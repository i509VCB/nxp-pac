#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e5ab29f 2026-04-30))"]
#[doc = "SYSCON."]
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
    #[doc = "AHB Matrix Priority Control."]
    #[inline(always)]
    pub const fn ahbmatprio(self) -> crate::pac::common::Reg<Ahbmatprio, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Secure CPU0 System Tick Calibration."]
    #[inline(always)]
    pub const fn cpu0stckcal(self) -> crate::pac::common::Reg<Cpu0stckcal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Non-Secure CPU0 System Tick Calibration."]
    #[inline(always)]
    pub const fn cpu0nstckcal(
        self,
    ) -> crate::pac::common::Reg<Cpu0nstckcal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "System tick calibration for CPU1."]
    #[inline(always)]
    pub const fn cpu1stckcal(self) -> crate::pac::common::Reg<Cpu1stckcal, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "NMI Source Select."]
    #[inline(always)]
    pub const fn nmisrc(self) -> crate::pac::common::Reg<Nmisrc, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Peripheral Reset Control 0."]
    #[inline(always)]
    pub const fn presetctrl0(self) -> crate::pac::common::Reg<Presetctrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Peripheral Reset Control 1."]
    #[inline(always)]
    pub const fn presetctrl1(self) -> crate::pac::common::Reg<Presetctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Peripheral Reset Control 2."]
    #[inline(always)]
    pub const fn presetctrl2(self) -> crate::pac::common::Reg<Presetctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Peripheral Reset Control 3."]
    #[inline(always)]
    pub const fn presetctrl3(self) -> crate::pac::common::Reg<Presetctrl3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Peripheral Reset Control Set."]
    #[inline(always)]
    pub const fn presetctrlset(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Presetctrlset, crate::pac::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize + n * 4usize) as _)
        }
    }
    #[doc = "Peripheral Reset Control Clear."]
    #[inline(always)]
    pub const fn presetctrlclr(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Presetctrlclr, crate::pac::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Clock Control 0."]
    #[inline(always)]
    pub const fn ahbclkctrl0(self) -> crate::pac::common::Reg<Ahbclkctrl0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "AHB Clock Control 1."]
    #[inline(always)]
    pub const fn ahbclkctrl1(self) -> crate::pac::common::Reg<Ahbclkctrl1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "AHB Clock Control 2."]
    #[inline(always)]
    pub const fn ahbclkctrl2(self) -> crate::pac::common::Reg<Ahbclkctrl2, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "AHB Clock Control 3."]
    #[inline(always)]
    pub const fn ahbclkctrl3(self) -> crate::pac::common::Reg<Ahbclkctrl3, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "AHB Clock Control Set."]
    #[inline(always)]
    pub const fn ahbclkctrlset(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ahbclkctrlset, crate::pac::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize + n * 4usize) as _)
        }
    }
    #[doc = "AHB Clock Control Clear."]
    #[inline(always)]
    pub const fn ahbclkctrlclr(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ahbclkctrlclr, crate::pac::common::W> {
        assert!(n < 4usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize + n * 4usize) as _)
        }
    }
    #[doc = "CPU0 System Tick Timer Source Select."]
    #[inline(always)]
    pub const fn systickclksel0(
        self,
    ) -> crate::pac::common::Reg<Systickclksel0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "CPU1 System Tick Timer Source Select."]
    #[inline(always)]
    pub const fn systickclksel1(
        self,
    ) -> crate::pac::common::Reg<Systickclksel1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "Trace Clock Source Select."]
    #[inline(always)]
    pub const fn traceclksel(self) -> crate::pac::common::Reg<Traceclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "CTIMER Clock Source Select."]
    #[inline(always)]
    pub const fn ctimerclksel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimerclksel, crate::pac::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize + n * 4usize) as _)
        }
    }
    #[doc = "CLKOUT Clock Source Select."]
    #[inline(always)]
    pub const fn clkoutsel(self) -> crate::pac::common::Reg<Clkoutsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "ADC0 Clock Source Select."]
    #[inline(always)]
    pub const fn adc0clksel(self) -> crate::pac::common::Reg<Adc0clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "USB-FS Clock Source Select."]
    #[inline(always)]
    pub const fn usb0clksel(self) -> crate::pac::common::Reg<Usb0clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn fcclksel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Fcclksel, crate::pac::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize + n * 4usize) as _)
        }
    }
    #[doc = "SCTimer/PWM Clock Source Select."]
    #[inline(always)]
    pub const fn sctclksel(self) -> crate::pac::common::Reg<Sctclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "CPU0 System Tick Timer Divider."]
    #[inline(always)]
    pub const fn systickclkdiv0(
        self,
    ) -> crate::pac::common::Reg<Systickclkdiv0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "CPU1 System Tick Timer Divider."]
    #[inline(always)]
    pub const fn systickclkdiv1(
        self,
    ) -> crate::pac::common::Reg<Systickclkdiv1, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "TRACE Clock Divider."]
    #[inline(always)]
    pub const fn traceclkdiv(self) -> crate::pac::common::Reg<Traceclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "TSI Function Clock Source Select."]
    #[inline(always)]
    pub const fn tsiclksel(self) -> crate::pac::common::Reg<Tsiclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "SINC FILTER Function Clock Source Select."]
    #[inline(always)]
    pub const fn sincfiltclksel(
        self,
    ) -> crate::pac::common::Reg<Sincfiltclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "SLOW_CLK Clock Divider."]
    #[inline(always)]
    pub const fn slowclkdiv(self) -> crate::pac::common::Reg<Slowclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "TSI Function Clock Divider."]
    #[inline(always)]
    pub const fn tsiclkdiv(self) -> crate::pac::common::Reg<Tsiclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "System Clock Divider."]
    #[inline(always)]
    pub const fn ahbclkdiv(self) -> crate::pac::common::Reg<Ahbclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "CLKOUT Clock Divider."]
    #[inline(always)]
    pub const fn clkoutdiv(self) -> crate::pac::common::Reg<Clkoutdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "FRO_HF_DIV Clock Divider."]
    #[inline(always)]
    pub const fn frohfdiv(self) -> crate::pac::common::Reg<Frohfdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "WDT0 Clock Divider."]
    #[inline(always)]
    pub const fn wdt0clkdiv(self) -> crate::pac::common::Reg<Wdt0clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "ADC0 Clock Divider."]
    #[inline(always)]
    pub const fn adc0clkdiv(self) -> crate::pac::common::Reg<Adc0clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "USB-FS Clock Divider."]
    #[inline(always)]
    pub const fn usb0clkdiv(self) -> crate::pac::common::Reg<Usb0clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "SCT/PWM Clock Divider."]
    #[inline(always)]
    pub const fn sctclkdiv(self) -> crate::pac::common::Reg<Sctclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "PLL Clock Divider."]
    #[inline(always)]
    pub const fn pllclkdiv(self) -> crate::pac::common::Reg<Pllclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "CTimer Clock Divider."]
    #[inline(always)]
    pub const fn ctimerclkdiv(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Ctimerclkdiv, crate::pac::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize + n * 4usize) as _)
        }
    }
    #[doc = "PLL1 Clock 0 Divider."]
    #[inline(always)]
    pub const fn pll1clk0div(self) -> crate::pac::common::Reg<Pll1clk0div, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "PLL1 Clock 1 Divider."]
    #[inline(always)]
    pub const fn pll1clk1div(self) -> crate::pac::common::Reg<Pll1clk1div, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "UTICK Clock Divider."]
    #[inline(always)]
    pub const fn utickclkdiv(self) -> crate::pac::common::Reg<Utickclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "CLKOUT FRG Control."]
    #[inline(always)]
    pub const fn clkout_frgctrl(
        self,
    ) -> crate::pac::common::Reg<ClkoutFrgctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Clock Configuration Unlock."]
    #[inline(always)]
    pub const fn clkunlock(self) -> crate::pac::common::Reg<Clkunlock, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "NVM Control."]
    #[inline(always)]
    pub const fn nvm_ctrl(self) -> crate::pac::common::Reg<NvmCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "ROM Wait State."]
    #[inline(always)]
    pub const fn romcr(self) -> crate::pac::common::Reg<Romcr, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "SmartDMA Interrupt Hijack."]
    #[inline(always)]
    pub const fn smart_dmaint(
        self,
    ) -> crate::pac::common::Reg<SmartDmaint, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "ADC1 Clock Source Select."]
    #[inline(always)]
    pub const fn adc1clksel(self) -> crate::pac::common::Reg<Adc1clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "ADC1 Clock Divider."]
    #[inline(always)]
    pub const fn adc1clkdiv(self) -> crate::pac::common::Reg<Adc1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "Control PKC RAM Interleave Access."]
    #[inline(always)]
    pub const fn ram_interleave(
        self,
    ) -> crate::pac::common::Reg<RamInterleave, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "DAC Functional Clock Selection."]
    #[inline(always)]
    pub const fn dacclksel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Dacclksel, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize + n * 8usize) as _)
        }
    }
    #[doc = "DAC functional clock divider."]
    #[inline(always)]
    pub const fn dacclkdiv(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Dacclkdiv, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize + n * 8usize) as _)
        }
    }
    #[doc = "FlexSPI Clock Selection."]
    #[inline(always)]
    pub const fn flex_spiclksel(
        self,
    ) -> crate::pac::common::Reg<FlexSpiclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "FlexSPI Clock Divider."]
    #[inline(always)]
    pub const fn flex_spiclkdiv(
        self,
    ) -> crate::pac::common::Reg<FlexSpiclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x04acusize) as _) }
    }
    #[doc = "PLL Clock Divider Clock Selection."]
    #[inline(always)]
    pub const fn pllclkdivsel(
        self,
    ) -> crate::pac::common::Reg<Pllclkdivsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "I3C0 Functional Clock Selection."]
    #[inline(always)]
    pub const fn i3c0fclksel(self) -> crate::pac::common::Reg<I3c0fclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "I3C0 FCLK_STC Clock Selection."]
    #[inline(always)]
    pub const fn i3c0fclkstcsel(
        self,
    ) -> crate::pac::common::Reg<I3c0fclkstcsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "I3C0 FCLK_STC Clock Divider."]
    #[inline(always)]
    pub const fn i3c0fclkstcdiv(
        self,
    ) -> crate::pac::common::Reg<I3c0fclkstcdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "I3C0 FCLK Slow Clock Divider."]
    #[inline(always)]
    pub const fn i3c0fclksdiv(
        self,
    ) -> crate::pac::common::Reg<I3c0fclksdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "I3C0 Functional Clock FCLK Divider."]
    #[inline(always)]
    pub const fn i3c0fclkdiv(self) -> crate::pac::common::Reg<I3c0fclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "I3C0 FCLK Slow Selection."]
    #[inline(always)]
    pub const fn i3c0fclkssel(
        self,
    ) -> crate::pac::common::Reg<I3c0fclkssel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "MICFIL Clock Selection."]
    #[inline(always)]
    pub const fn micfilfclksel(
        self,
    ) -> crate::pac::common::Reg<Micfilfclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "MICFIL Clock Division."]
    #[inline(always)]
    pub const fn micfilfclkdiv(
        self,
    ) -> crate::pac::common::Reg<Micfilfclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "uSDHC Clock Selection."]
    #[inline(always)]
    pub const fn u_sdhcclksel(
        self,
    ) -> crate::pac::common::Reg<USdhcclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "uSDHC Function Clock Divider."]
    #[inline(always)]
    pub const fn u_sdhcclkdiv(
        self,
    ) -> crate::pac::common::Reg<USdhcclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "FLEXIO Clock Selection."]
    #[inline(always)]
    pub const fn flexioclksel(
        self,
    ) -> crate::pac::common::Reg<Flexioclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "FLEXIO Function Clock Divider."]
    #[inline(always)]
    pub const fn flexioclkdiv(
        self,
    ) -> crate::pac::common::Reg<Flexioclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "FLEXCAN0 Clock Selection."]
    #[inline(always)]
    pub const fn flexcan0clksel(
        self,
    ) -> crate::pac::common::Reg<Flexcan0clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "FLEXCAN0 Function Clock Divider."]
    #[inline(always)]
    pub const fn flexcan0clkdiv(
        self,
    ) -> crate::pac::common::Reg<Flexcan0clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "FLEXCAN1 Clock Selection."]
    #[inline(always)]
    pub const fn flexcan1clksel(
        self,
    ) -> crate::pac::common::Reg<Flexcan1clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "FLEXCAN1 Function Clock Divider."]
    #[inline(always)]
    pub const fn flexcan1clkdiv(
        self,
    ) -> crate::pac::common::Reg<Flexcan1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "Ethernet RMII Clock Selection."]
    #[inline(always)]
    pub const fn enetrmiiclksel(
        self,
    ) -> crate::pac::common::Reg<Enetrmiiclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "Ethernet RMII Function Clock Divider."]
    #[inline(always)]
    pub const fn enetrmiiclkdiv(
        self,
    ) -> crate::pac::common::Reg<Enetrmiiclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "Ethernet PTP REF Clock Selection."]
    #[inline(always)]
    pub const fn enetptprefclksel(
        self,
    ) -> crate::pac::common::Reg<Enetptprefclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "Ethernet PTP REF Function Clock Divider."]
    #[inline(always)]
    pub const fn enetptprefclkdiv(
        self,
    ) -> crate::pac::common::Reg<Enetptprefclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "Ethernet PHY Interface Select."]
    #[inline(always)]
    pub const fn enet_phy_intf_sel(
        self,
    ) -> crate::pac::common::Reg<EnetPhyIntfSel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "Sideband Flow Control."]
    #[inline(always)]
    pub const fn enet_sbd_flow_ctrl(
        self,
    ) -> crate::pac::common::Reg<EnetSbdFlowCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "EWM0 Clock Selection."]
    #[inline(always)]
    pub const fn ewm0clksel(self) -> crate::pac::common::Reg<Ewm0clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "WDT1 Clock Selection."]
    #[inline(always)]
    pub const fn wdt1clksel(self) -> crate::pac::common::Reg<Wdt1clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "WDT1 Function Clock Divider."]
    #[inline(always)]
    pub const fn wdt1clkdiv(self) -> crate::pac::common::Reg<Wdt1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "OSTIMER Clock Selection."]
    #[inline(always)]
    pub const fn ostimerclksel(
        self,
    ) -> crate::pac::common::Reg<Ostimerclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "CMP Function Clock Selection."]
    #[inline(always)]
    pub const fn cmpfclksel(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Cmpfclksel, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize + n * 16usize) as _)
        }
    }
    #[doc = "CMP Function Clock Divider."]
    #[inline(always)]
    pub const fn cmpfclkdiv(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Cmpfclkdiv, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize + n * 16usize) as _)
        }
    }
    #[doc = "CMP0 Round Robin Clock Selection."]
    #[inline(always)]
    pub const fn cmp0rrclksel(
        self,
    ) -> crate::pac::common::Reg<Cmp0rrclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "CMP Round Robin Clock Divider."]
    #[inline(always)]
    pub const fn cmprrclkdiv(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Cmprrclkdiv, crate::pac::common::RW> {
        assert!(n < 3usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize + n * 16usize) as _)
        }
    }
    #[doc = "CMP1 Round Robin Clock Source Select."]
    #[inline(always)]
    pub const fn cmp1rrclksel(
        self,
    ) -> crate::pac::common::Reg<Cmp1rrclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "CMP2 Round Robin Clock Source Select."]
    #[inline(always)]
    pub const fn cmp2rrclksel(
        self,
    ) -> crate::pac::common::Reg<Cmp2rrclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "CPU Control for Multiple Processors."]
    #[inline(always)]
    pub const fn cpuctrl(self) -> crate::pac::common::Reg<Cpuctrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Coprocessor Boot Address."]
    #[inline(always)]
    pub const fn cpboot(self) -> crate::pac::common::Reg<Cpboot, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "CPU Status."]
    #[inline(always)]
    pub const fn cpustat(self) -> crate::pac::common::Reg<Cpustat, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
    #[doc = "LPCAC Control."]
    #[inline(always)]
    pub const fn lpcac_ctrl(self) -> crate::pac::common::Reg<LpcacCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0824usize) as _) }
    }
    #[doc = "LP_FLEXCOMM Clock Divider."]
    #[inline(always)]
    pub const fn flexcommclkdiv(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<Flexcommclkdiv, crate::pac::common::RW> {
        assert!(n < 10usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0850usize + n * 4usize) as _)
        }
    }
    #[doc = "UTICK Function Clock Source Select."]
    #[inline(always)]
    pub const fn utickclksel(self) -> crate::pac::common::Reg<Utickclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0878usize) as _) }
    }
    #[doc = "SAI0 Function Clock Source Select."]
    #[inline(always)]
    pub const fn sai0clksel(self) -> crate::pac::common::Reg<Sai0clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "SAI1 Function Clock Source Select."]
    #[inline(always)]
    pub const fn sai1clksel(self) -> crate::pac::common::Reg<Sai1clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0884usize) as _) }
    }
    #[doc = "SAI0 Function Clock Division."]
    #[inline(always)]
    pub const fn sai0clkdiv(self) -> crate::pac::common::Reg<Sai0clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0888usize) as _) }
    }
    #[doc = "SAI1 Function Clock Division."]
    #[inline(always)]
    pub const fn sai1clkdiv(self) -> crate::pac::common::Reg<Sai1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x088cusize) as _) }
    }
    #[doc = "EMVSIM0 Clock Source Select."]
    #[inline(always)]
    pub const fn emvsim0clksel(
        self,
    ) -> crate::pac::common::Reg<Emvsim0clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0890usize) as _) }
    }
    #[doc = "EMVSIM1 Clock Source Select."]
    #[inline(always)]
    pub const fn emvsim1clksel(
        self,
    ) -> crate::pac::common::Reg<Emvsim1clksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0894usize) as _) }
    }
    #[doc = "EMVSIM0 Function Clock Division."]
    #[inline(always)]
    pub const fn emvsim0clkdiv(
        self,
    ) -> crate::pac::common::Reg<Emvsim0clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0898usize) as _) }
    }
    #[doc = "EMVSIM1 Function Clock Division."]
    #[inline(always)]
    pub const fn emvsim1clkdiv(
        self,
    ) -> crate::pac::common::Reg<Emvsim1clkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x089cusize) as _) }
    }
    #[doc = "Key Retain Control."]
    #[inline(always)]
    pub const fn key_retain_ctrl(
        self,
    ) -> crate::pac::common::Reg<KeyRetainCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0950usize) as _) }
    }
    #[doc = "FRO 48MHz Reference Clock Control."]
    #[inline(always)]
    pub const fn ref_clk_ctrl(self) -> crate::pac::common::Reg<RefClkCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[doc = "FRO 48MHz Reference Clock Control Set."]
    #[inline(always)]
    pub const fn ref_clk_ctrl_set(
        self,
    ) -> crate::pac::common::Reg<RefClkCtrlSet, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0964usize) as _) }
    }
    #[doc = "FRO 48MHz Reference Clock Control Clear."]
    #[inline(always)]
    pub const fn ref_clk_ctrl_clr(
        self,
    ) -> crate::pac::common::Reg<RefClkCtrlClr, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0968usize) as _) }
    }
    #[doc = "GDET Control Register."]
    #[inline(always)]
    pub const fn gdet_ctrl(
        self,
        n: usize,
    ) -> crate::pac::common::Reg<GdetCtrl, crate::pac::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x096cusize + n * 4usize) as _)
        }
    }
    #[doc = "ELS Asset Protection Register."]
    #[inline(always)]
    pub const fn els_asset_prot(
        self,
    ) -> crate::pac::common::Reg<ElsAssetProt, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0974usize) as _) }
    }
    #[doc = "ELS Lock Control."]
    #[inline(always)]
    pub const fn els_lock_ctrl(
        self,
    ) -> crate::pac::common::Reg<ElsLockCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0978usize) as _) }
    }
    #[doc = "ELS Lock Control DP."]
    #[inline(always)]
    pub const fn els_lock_ctrl_dp(
        self,
    ) -> crate::pac::common::Reg<ElsLockCtrlDp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x097cusize) as _) }
    }
    #[doc = "Life Cycle State Register."]
    #[inline(always)]
    pub const fn els_otp_lc_state(
        self,
    ) -> crate::pac::common::Reg<ElsOtpLcState, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "Life Cycle State Register (Duplicate)."]
    #[inline(always)]
    pub const fn els_otp_lc_state_dp(
        self,
    ) -> crate::pac::common::Reg<ElsOtpLcStateDp, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0984usize) as _) }
    }
    #[doc = "ELS Temporal State."]
    #[inline(always)]
    pub const fn els_temporal_state(
        self,
    ) -> crate::pac::common::Reg<ElsTemporalState, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0988usize) as _) }
    }
    #[doc = "Key Derivation Function Mask."]
    #[inline(always)]
    pub const fn els_kdf_mask(self) -> crate::pac::common::Reg<ElsKdfMask, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x098cusize) as _) }
    }
    #[doc = "ELS AS Configuration."]
    #[inline(always)]
    pub const fn els_as_cfg0(self) -> crate::pac::common::Reg<ElsAsCfg0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d0usize) as _) }
    }
    #[doc = "ELS AS Configuration1."]
    #[inline(always)]
    pub const fn els_as_cfg1(self) -> crate::pac::common::Reg<ElsAsCfg1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d4usize) as _) }
    }
    #[doc = "ELS AS Configuration2."]
    #[inline(always)]
    pub const fn els_as_cfg2(self) -> crate::pac::common::Reg<ElsAsCfg2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09d8usize) as _) }
    }
    #[doc = "ELS AS Configuration3."]
    #[inline(always)]
    pub const fn els_as_cfg3(self) -> crate::pac::common::Reg<ElsAsCfg3, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09dcusize) as _) }
    }
    #[doc = "ELS AS State Register."]
    #[inline(always)]
    pub const fn els_as_st0(self) -> crate::pac::common::Reg<ElsAsSt0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[doc = "ELS AS State1."]
    #[inline(always)]
    pub const fn els_as_st1(self) -> crate::pac::common::Reg<ElsAsSt1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e4usize) as _) }
    }
    #[doc = "Boot state captured during boot: Main ROM log."]
    #[inline(always)]
    pub const fn els_as_boot_log0(
        self,
    ) -> crate::pac::common::Reg<ElsAsBootLog0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e8usize) as _) }
    }
    #[doc = "Boot state captured during boot: Library log."]
    #[inline(always)]
    pub const fn els_as_boot_log1(
        self,
    ) -> crate::pac::common::Reg<ElsAsBootLog1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09ecusize) as _) }
    }
    #[doc = "Boot state captured during boot: Hardware status signals log."]
    #[inline(always)]
    pub const fn els_as_boot_log2(
        self,
    ) -> crate::pac::common::Reg<ElsAsBootLog2, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f0usize) as _) }
    }
    #[doc = "Boot state captured during boot: Security log."]
    #[inline(always)]
    pub const fn els_as_boot_log3(
        self,
    ) -> crate::pac::common::Reg<ElsAsBootLog3, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f4usize) as _) }
    }
    #[doc = "ELS AS Flag0."]
    #[inline(always)]
    pub const fn els_as_flag0(self) -> crate::pac::common::Reg<ElsAsFlag0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09f8usize) as _) }
    }
    #[doc = "ELS AS Flag1."]
    #[inline(always)]
    pub const fn els_as_flag1(self) -> crate::pac::common::Reg<ElsAsFlag1, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x09fcusize) as _) }
    }
    #[doc = "Clock Control."]
    #[inline(always)]
    pub const fn clock_ctrl(self) -> crate::pac::common::Reg<ClockCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a18usize) as _) }
    }
    #[doc = "I3C1 Functional Clock Selection."]
    #[inline(always)]
    pub const fn i3c1fclksel(self) -> crate::pac::common::Reg<I3c1fclksel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b30usize) as _) }
    }
    #[doc = "Selects the I3C1 Time Control clock."]
    #[inline(always)]
    pub const fn i3c1fclkstcsel(
        self,
    ) -> crate::pac::common::Reg<I3c1fclkstcsel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b34usize) as _) }
    }
    #[doc = "I3C1 FCLK_STC Clock Divider."]
    #[inline(always)]
    pub const fn i3c1fclkstcdiv(
        self,
    ) -> crate::pac::common::Reg<I3c1fclkstcdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b38usize) as _) }
    }
    #[doc = "I3C1 FCLK Slow clock Divider."]
    #[inline(always)]
    pub const fn i3c1fclksdiv(
        self,
    ) -> crate::pac::common::Reg<I3c1fclksdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b3cusize) as _) }
    }
    #[doc = "I3C1 Functional Clock FCLK Divider."]
    #[inline(always)]
    pub const fn i3c1fclkdiv(self) -> crate::pac::common::Reg<I3c1fclkdiv, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "I3C1 FCLK Slow Selection."]
    #[inline(always)]
    pub const fn i3c1fclkssel(
        self,
    ) -> crate::pac::common::Reg<I3c1fclkssel, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b44usize) as _) }
    }
    #[doc = "ETB Counter Status Register."]
    #[inline(always)]
    pub const fn etb_status(self) -> crate::pac::common::Reg<EtbStatus, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b50usize) as _) }
    }
    #[doc = "ETB Counter Control Register."]
    #[inline(always)]
    pub const fn etb_counter_ctrl(
        self,
    ) -> crate::pac::common::Reg<EtbCounterCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b54usize) as _) }
    }
    #[doc = "ETB Counter Reload Register."]
    #[inline(always)]
    pub const fn etb_counter_reload(
        self,
    ) -> crate::pac::common::Reg<EtbCounterReload, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b58usize) as _) }
    }
    #[doc = "ETB Counter Value Register."]
    #[inline(always)]
    pub const fn etb_counter_value(
        self,
    ) -> crate::pac::common::Reg<EtbCounterValue, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b5cusize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray code_gray\\[31:0\\]."]
    #[inline(always)]
    pub const fn gray_code_lsb(
        self,
    ) -> crate::pac::common::Reg<GrayCodeLsb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "Gray to Binary Converter Gray code_gray\\[41:32\\]."]
    #[inline(always)]
    pub const fn gray_code_msb(
        self,
    ) -> crate::pac::common::Reg<GrayCodeMsb, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b64usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[31:0\\]."]
    #[inline(always)]
    pub const fn binary_code_lsb(
        self,
    ) -> crate::pac::common::Reg<BinaryCodeLsb, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b68usize) as _) }
    }
    #[doc = "Gray to Binary Converter Binary Code \\[41:32\\]."]
    #[inline(always)]
    pub const fn binary_code_msb(
        self,
    ) -> crate::pac::common::Reg<BinaryCodeMsb, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b6cusize) as _) }
    }
    #[doc = "Control Automatic Clock Gating."]
    #[inline(always)]
    pub const fn autoclkgateoverride(
        self,
    ) -> crate::pac::common::Reg<Autoclkgateoverride, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e04usize) as _) }
    }
    #[doc = "Control Automatic Clock Gating C."]
    #[inline(always)]
    pub const fn autoclkgateoverridec(
        self,
    ) -> crate::pac::common::Reg<Autoclkgateoverridec, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e2cusize) as _) }
    }
    #[doc = "PWM0 Submodule Control."]
    #[inline(always)]
    pub const fn pwm0subctl(self) -> crate::pac::common::Reg<Pwm0subctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e38usize) as _) }
    }
    #[doc = "PWM1 Submodule Control."]
    #[inline(always)]
    pub const fn pwm1subctl(self) -> crate::pac::common::Reg<Pwm1subctl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e3cusize) as _) }
    }
    #[doc = "CTIMER Global Start Enable."]
    #[inline(always)]
    pub const fn ctimerglobalstarten(
        self,
    ) -> crate::pac::common::Reg<Ctimerglobalstarten, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e40usize) as _) }
    }
    #[doc = "RAM ECC Enable Control."]
    #[inline(always)]
    pub const fn ecc_enable_ctrl(
        self,
    ) -> crate::pac::common::Reg<EccEnableCtrl, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0e44usize) as _) }
    }
    #[doc = "Control Write Access to Security."]
    #[inline(always)]
    pub const fn debug_lock_en(
        self,
    ) -> crate::pac::common::Reg<DebugLockEn, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa0usize) as _) }
    }
    #[doc = "Cortex Debug Features Control."]
    #[inline(always)]
    pub const fn debug_features(
        self,
    ) -> crate::pac::common::Reg<DebugFeatures, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa4usize) as _) }
    }
    #[doc = "Cortex Debug Features Control (Duplicate)."]
    #[inline(always)]
    pub const fn debug_features_dp(
        self,
    ) -> crate::pac::common::Reg<DebugFeaturesDp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fa8usize) as _) }
    }
    #[doc = "CPU0 Software Debug Access."]
    #[inline(always)]
    pub const fn swd_access_cpu0(
        self,
    ) -> crate::pac::common::Reg<SwdAccessCpu0, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb4usize) as _) }
    }
    #[doc = "CPU1 Software Debug Access."]
    #[inline(always)]
    pub const fn swd_access_cpu1(
        self,
    ) -> crate::pac::common::Reg<SwdAccessCpu1, crate::pac::common::W> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fb8usize) as _) }
    }
    #[doc = "Debug Authentication BEACON."]
    #[inline(always)]
    pub const fn debug_auth_beacon(
        self,
    ) -> crate::pac::common::Reg<DebugAuthBeacon, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc0usize) as _) }
    }
    #[doc = "DSP Software Debug Access."]
    #[inline(always)]
    pub const fn swd_access_dsp(
        self,
    ) -> crate::pac::common::Reg<SwdAccessDsp, crate::pac::common::RW> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fc4usize) as _) }
    }
    #[doc = "JTAG Chip ID."]
    #[inline(always)]
    pub const fn jtag_id(self) -> crate::pac::common::Reg<JtagId, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "Device Type."]
    #[inline(always)]
    pub const fn device_type(self) -> crate::pac::common::Reg<DeviceType, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Device ID."]
    #[inline(always)]
    pub const fn device_id0(self) -> crate::pac::common::Reg<DeviceId0, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Chip Revision ID and Number."]
    #[inline(always)]
    pub const fn dieid(self) -> crate::pac::common::Reg<Dieid, crate::pac::common::R> {
        unsafe { crate::pac::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
#[doc = "ADC0 Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0clkdiv(pub u32);
impl Adc0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Adc0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Adc0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Adc0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Adc0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Adc0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Adc0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Adc0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Adc0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Adc0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Adc0clkdiv {
    #[inline(always)]
    fn default() -> Adc0clkdiv {
        Adc0clkdiv(0)
    }
}
impl core::fmt::Debug for Adc0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC0 Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0clksel(pub u32);
impl Adc0clksel {
    #[doc = "Selects the ADC0 clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Adc0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Adc0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the ADC0 clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Adc0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc0clksel {
    #[inline(always)]
    fn default() -> Adc0clksel {
        Adc0clksel(0)
    }
}
impl core::fmt::Debug for Adc0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "ADC1 Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1clkdiv(pub u32);
impl Adc1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Adc1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Adc1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Adc1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Adc1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Adc1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Adc1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Adc1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Adc1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Adc1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Adc1clkdiv {
    #[inline(always)]
    fn default() -> Adc1clkdiv {
        Adc1clkdiv(0)
    }
}
impl core::fmt::Debug for Adc1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "ADC1 Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1clksel(pub u32);
impl Adc1clksel {
    #[doc = "Selects the ADC1 clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Adc1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Adc1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the ADC1 clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Adc1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc1clksel {
    #[inline(always)]
    fn default() -> Adc1clksel {
        Adc1clksel(0)
    }
}
impl core::fmt::Debug for Adc1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Adc1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "AHB Clock Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl0(pub u32);
impl Ahbclkctrl0 {
    #[doc = "Enables the clock for the ROM."]
    #[must_use]
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the ROM."]
    #[inline(always)]
    pub const fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for the RAMB Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMB Controller."]
    #[inline(always)]
    pub const fn set_ramb_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for the RAMC Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_ctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMC Controller."]
    #[inline(always)]
    pub const fn set_ramc_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for the RAMD Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramd_ctrl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMD Controller."]
    #[inline(always)]
    pub const fn set_ramd_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for the RAME Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn rame_ctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAME Controller."]
    #[inline(always)]
    pub const fn set_rame_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for the RAMF Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramf_ctrl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMF Controller."]
    #[inline(always)]
    pub const fn set_ramf_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for the RAMG Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramg_ctrl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMG Controller."]
    #[inline(always)]
    pub const fn set_ramg_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the RAMH Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramh_ctrl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the RAMH Controller."]
    #[inline(always)]
    pub const fn set_ramh_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for the Flash Management Unit."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Flash Management Unit."]
    #[inline(always)]
    pub const fn set_fmu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enables the clock for the Flash Memory Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn fmc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Flash Memory Controller."]
    #[inline(always)]
    pub const fn set_fmc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables the clock for FlexSPI."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for FlexSPI."]
    #[inline(always)]
    pub const fn set_flexspi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for INPUTMUX."]
    #[must_use]
    #[inline(always)]
    pub const fn mux(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for INPUTMUX."]
    #[inline(always)]
    pub const fn set_mux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock for the PORT controller."]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self, n: usize) -> bool {
        assert!(n < 5usize);
        let offs = 13usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the PORT controller."]
    #[inline(always)]
    pub const fn set_port(&mut self, n: usize, val: bool) {
        assert!(n < 5usize);
        let offs = 13usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Enables the clock for GPIO."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio(&self, n: usize) -> bool {
        assert!(n < 5usize);
        let offs = 19usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GPIO."]
    #[inline(always)]
    pub const fn set_gpio(&mut self, n: usize, val: bool) {
        assert!(n < 5usize);
        let offs = 19usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Enables the clock for PINT."]
    #[must_use]
    #[inline(always)]
    pub const fn pint(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PINT."]
    #[inline(always)]
    pub const fn set_pint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for DMA0."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DMA0."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for CRC."]
    #[must_use]
    #[inline(always)]
    pub const fn crc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CRC."]
    #[inline(always)]
    pub const fn set_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enables the clock for WWDT0."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt0(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for WWDT0."]
    #[inline(always)]
    pub const fn set_wwdt0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enables the clock for WWDT1."]
    #[must_use]
    #[inline(always)]
    pub const fn wwdt1(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for WWDT1."]
    #[inline(always)]
    pub const fn set_wwdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Inter CPU communication Mailbox."]
    #[inline(always)]
    pub const fn set_mailbox(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkctrl0 {
    #[inline(always)]
    fn default() -> Ahbclkctrl0 {
        Ahbclkctrl0(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl0")
            .field("rom", &self.rom())
            .field("ramb_ctrl", &self.ramb_ctrl())
            .field("ramc_ctrl", &self.ramc_ctrl())
            .field("ramd_ctrl", &self.ramd_ctrl())
            .field("rame_ctrl", &self.rame_ctrl())
            .field("ramf_ctrl", &self.ramf_ctrl())
            .field("ramg_ctrl", &self.ramg_ctrl())
            .field("ramh_ctrl", &self.ramh_ctrl())
            .field("fmu", &self.fmu())
            .field("fmc", &self.fmc())
            .field("flexspi", &self.flexspi())
            .field("mux", &self.mux())
            .field("port[0]", &self.port(0usize))
            .field("port[1]", &self.port(1usize))
            .field("port[2]", &self.port(2usize))
            .field("port[3]", &self.port(3usize))
            .field("port[4]", &self.port(4usize))
            .field("gpio[0]", &self.gpio(0usize))
            .field("gpio[1]", &self.gpio(1usize))
            .field("gpio[2]", &self.gpio(2usize))
            .field("gpio[3]", &self.gpio(3usize))
            .field("gpio[4]", &self.gpio(4usize))
            .field("pint", &self.pint())
            .field("dma0", &self.dma0())
            .field("crc", &self.crc())
            .field("wwdt0", &self.wwdt0())
            .field("wwdt1", &self.wwdt1())
            .field("mailbox", &self.mailbox())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl0 {{ rom: {=bool:?}, ramb_ctrl: {=bool:?}, ramc_ctrl: {=bool:?}, ramd_ctrl: {=bool:?}, rame_ctrl: {=bool:?}, ramf_ctrl: {=bool:?}, ramg_ctrl: {=bool:?}, ramh_ctrl: {=bool:?}, fmu: {=bool:?}, fmc: {=bool:?}, flexspi: {=bool:?}, mux: {=bool:?}, port[0]: {=bool:?}, port[1]: {=bool:?}, port[2]: {=bool:?}, port[3]: {=bool:?}, port[4]: {=bool:?}, gpio[0]: {=bool:?}, gpio[1]: {=bool:?}, gpio[2]: {=bool:?}, gpio[3]: {=bool:?}, gpio[4]: {=bool:?}, pint: {=bool:?}, dma0: {=bool:?}, crc: {=bool:?}, wwdt0: {=bool:?}, wwdt1: {=bool:?}, mailbox: {=bool:?} }}",
            self.rom(),
            self.ramb_ctrl(),
            self.ramc_ctrl(),
            self.ramd_ctrl(),
            self.rame_ctrl(),
            self.ramf_ctrl(),
            self.ramg_ctrl(),
            self.ramh_ctrl(),
            self.fmu(),
            self.fmc(),
            self.flexspi(),
            self.mux(),
            self.port(0usize),
            self.port(1usize),
            self.port(2usize),
            self.port(3usize),
            self.port(4usize),
            self.gpio(0usize),
            self.gpio(1usize),
            self.gpio(2usize),
            self.gpio(3usize),
            self.gpio(4usize),
            self.pint(),
            self.dma0(),
            self.crc(),
            self.wwdt0(),
            self.wwdt1(),
            self.mailbox()
        )
    }
}
#[doc = "AHB Clock Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl1(pub u32);
impl Ahbclkctrl1 {
    #[doc = "Enables the clock for MRT."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for MRT."]
    #[inline(always)]
    pub const fn set_mrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the clock for the OS Event Timer."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the OS Event Timer."]
    #[inline(always)]
    pub const fn set_ostimer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for SCT."]
    #[must_use]
    #[inline(always)]
    pub const fn sct(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SCT."]
    #[inline(always)]
    pub const fn set_sct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for ADC0."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ADC0."]
    #[inline(always)]
    pub const fn set_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for ADC1."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ADC1."]
    #[inline(always)]
    pub const fn set_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for DAC0."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DAC0."]
    #[inline(always)]
    pub const fn set_dac0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for RTC."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for RTC."]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for EVSIM0."]
    #[must_use]
    #[inline(always)]
    pub const fn evsim0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EVSIM0."]
    #[inline(always)]
    pub const fn set_evsim0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for EVSIM1."]
    #[must_use]
    #[inline(always)]
    pub const fn evsim1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EVSIM1."]
    #[inline(always)]
    pub const fn set_evsim1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enables the clock for UTICK."]
    #[must_use]
    #[inline(always)]
    pub const fn utick(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for UTICK."]
    #[inline(always)]
    pub const fn set_utick(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables the clock for LP_FLEXCOMM."]
    #[must_use]
    #[inline(always)]
    pub const fn fc(&self, n: usize) -> bool {
        assert!(n < 10usize);
        let offs = 11usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for LP_FLEXCOMM."]
    #[inline(always)]
    pub const fn set_fc(&mut self, n: usize, val: bool) {
        assert!(n < 10usize);
        let offs = 11usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Enables the clock for MICFIL."]
    #[must_use]
    #[inline(always)]
    pub const fn micfil(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for MICFIL."]
    #[inline(always)]
    pub const fn set_micfil(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for CTIMER2."]
    #[must_use]
    #[inline(always)]
    pub const fn timer2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER2."]
    #[inline(always)]
    pub const fn set_timer2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for USB-FS DCD."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs_dcd(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB-FS DCD."]
    #[inline(always)]
    pub const fn set_usb0_fs_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for USB-FS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB-FS."]
    #[inline(always)]
    pub const fn set_usb0_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for CTIMER0."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER0."]
    #[inline(always)]
    pub const fn set_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for CTIMER1."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER1."]
    #[inline(always)]
    pub const fn set_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Enables the clock for PKC RAM."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_ram(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PKC RAM."]
    #[inline(always)]
    pub const fn set_pkc_ram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for SmartDMA."]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SmartDMA."]
    #[inline(always)]
    pub const fn set_smart_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkctrl1 {
    #[inline(always)]
    fn default() -> Ahbclkctrl1 {
        Ahbclkctrl1(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl1")
            .field("mrt", &self.mrt())
            .field("ostimer", &self.ostimer())
            .field("sct", &self.sct())
            .field("adc0", &self.adc0())
            .field("adc1", &self.adc1())
            .field("dac0", &self.dac0())
            .field("rtc", &self.rtc())
            .field("evsim0", &self.evsim0())
            .field("evsim1", &self.evsim1())
            .field("utick", &self.utick())
            .field("fc[0]", &self.fc(0usize))
            .field("fc[1]", &self.fc(1usize))
            .field("fc[2]", &self.fc(2usize))
            .field("fc[3]", &self.fc(3usize))
            .field("fc[4]", &self.fc(4usize))
            .field("fc[5]", &self.fc(5usize))
            .field("fc[6]", &self.fc(6usize))
            .field("fc[7]", &self.fc(7usize))
            .field("fc[8]", &self.fc(8usize))
            .field("fc[9]", &self.fc(9usize))
            .field("micfil", &self.micfil())
            .field("timer2", &self.timer2())
            .field("usb0_fs_dcd", &self.usb0_fs_dcd())
            .field("usb0_fs", &self.usb0_fs())
            .field("timer0", &self.timer0())
            .field("timer1", &self.timer1())
            .field("pkc_ram", &self.pkc_ram())
            .field("smart_dma", &self.smart_dma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl1 {{ mrt: {=bool:?}, ostimer: {=bool:?}, sct: {=bool:?}, adc0: {=bool:?}, adc1: {=bool:?}, dac0: {=bool:?}, rtc: {=bool:?}, evsim0: {=bool:?}, evsim1: {=bool:?}, utick: {=bool:?}, fc[0]: {=bool:?}, fc[1]: {=bool:?}, fc[2]: {=bool:?}, fc[3]: {=bool:?}, fc[4]: {=bool:?}, fc[5]: {=bool:?}, fc[6]: {=bool:?}, fc[7]: {=bool:?}, fc[8]: {=bool:?}, fc[9]: {=bool:?}, micfil: {=bool:?}, timer2: {=bool:?}, usb0_fs_dcd: {=bool:?}, usb0_fs: {=bool:?}, timer0: {=bool:?}, timer1: {=bool:?}, pkc_ram: {=bool:?}, smart_dma: {=bool:?} }}",
            self.mrt(),
            self.ostimer(),
            self.sct(),
            self.adc0(),
            self.adc1(),
            self.dac0(),
            self.rtc(),
            self.evsim0(),
            self.evsim1(),
            self.utick(),
            self.fc(0usize),
            self.fc(1usize),
            self.fc(2usize),
            self.fc(3usize),
            self.fc(4usize),
            self.fc(5usize),
            self.fc(6usize),
            self.fc(7usize),
            self.fc(8usize),
            self.fc(9usize),
            self.micfil(),
            self.timer2(),
            self.usb0_fs_dcd(),
            self.usb0_fs(),
            self.timer0(),
            self.timer1(),
            self.pkc_ram(),
            self.smart_dma()
        )
    }
}
#[doc = "AHB Clock Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl2(pub u32);
impl Ahbclkctrl2 {
    #[doc = "Enables the clock for DMA1."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DMA1."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for Ethernet."]
    #[must_use]
    #[inline(always)]
    pub const fn enet(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Ethernet."]
    #[inline(always)]
    pub const fn set_enet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for uSDHC."]
    #[must_use]
    #[inline(always)]
    pub const fn u_sdhc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for uSDHC."]
    #[inline(always)]
    pub const fn set_u_sdhc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for Flexio."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Flexio."]
    #[inline(always)]
    pub const fn set_flexio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for SAI0."]
    #[must_use]
    #[inline(always)]
    pub const fn sai0(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SAI0."]
    #[inline(always)]
    pub const fn set_sai0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for SAI1."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SAI1."]
    #[inline(always)]
    pub const fn set_sai1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for TRO."]
    #[must_use]
    #[inline(always)]
    pub const fn tro(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for TRO."]
    #[inline(always)]
    pub const fn set_tro(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for the Frequency meter."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for the Frequency meter."]
    #[inline(always)]
    pub const fn set_freqme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for TRNG."]
    #[must_use]
    #[inline(always)]
    pub const fn trng(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for TRNG."]
    #[inline(always)]
    pub const fn set_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enables the clock for FLEXCAN0."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for FLEXCAN0."]
    #[inline(always)]
    pub const fn set_flexcan0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enables the clock for FLEXCAN1."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for FLEXCAN1."]
    #[inline(always)]
    pub const fn set_flexcan1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Enables the clock for USB HS."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB HS."]
    #[inline(always)]
    pub const fn set_usb_hs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables the clock for USB HS PHY."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_phy(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for USB HS PHY."]
    #[inline(always)]
    pub const fn set_usb_hs_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Enables the clock for ELS."]
    #[must_use]
    #[inline(always)]
    pub const fn els(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ELS."]
    #[inline(always)]
    pub const fn set_els(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for Powerquad."]
    #[must_use]
    #[inline(always)]
    pub const fn pq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Powerquad."]
    #[inline(always)]
    pub const fn set_pq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for PLU_LUT."]
    #[must_use]
    #[inline(always)]
    pub const fn plu_lut(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PLU_LUT."]
    #[inline(always)]
    pub const fn set_plu_lut(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for CTIMER3."]
    #[must_use]
    #[inline(always)]
    pub const fn timer3(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER3."]
    #[inline(always)]
    pub const fn set_timer3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for CTIMER4."]
    #[must_use]
    #[inline(always)]
    pub const fn timer4(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CTIMER4."]
    #[inline(always)]
    pub const fn set_timer4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for PUF."]
    #[must_use]
    #[inline(always)]
    pub const fn puf(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PUF."]
    #[inline(always)]
    pub const fn set_puf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for PKC."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PKC."]
    #[inline(always)]
    pub const fn set_pkc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for SCG."]
    #[must_use]
    #[inline(always)]
    pub const fn scg(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SCG."]
    #[inline(always)]
    pub const fn set_scg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for GDET0 and GDET1."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for GDET0 and GDET1."]
    #[inline(always)]
    pub const fn set_gdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enables the clock for SM3."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SM3."]
    #[inline(always)]
    pub const fn set_sm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Ahbclkctrl2 {
    #[inline(always)]
    fn default() -> Ahbclkctrl2 {
        Ahbclkctrl2(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl2")
            .field("dma1", &self.dma1())
            .field("enet", &self.enet())
            .field("u_sdhc", &self.u_sdhc())
            .field("flexio", &self.flexio())
            .field("sai0", &self.sai0())
            .field("sai1", &self.sai1())
            .field("tro", &self.tro())
            .field("freqme", &self.freqme())
            .field("trng", &self.trng())
            .field("flexcan0", &self.flexcan0())
            .field("flexcan1", &self.flexcan1())
            .field("usb_hs", &self.usb_hs())
            .field("usb_hs_phy", &self.usb_hs_phy())
            .field("els", &self.els())
            .field("pq", &self.pq())
            .field("plu_lut", &self.plu_lut())
            .field("timer3", &self.timer3())
            .field("timer4", &self.timer4())
            .field("puf", &self.puf())
            .field("pkc", &self.pkc())
            .field("scg", &self.scg())
            .field("gdet", &self.gdet())
            .field("sm3", &self.sm3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl2 {{ dma1: {=bool:?}, enet: {=bool:?}, u_sdhc: {=bool:?}, flexio: {=bool:?}, sai0: {=bool:?}, sai1: {=bool:?}, tro: {=bool:?}, freqme: {=bool:?}, trng: {=bool:?}, flexcan0: {=bool:?}, flexcan1: {=bool:?}, usb_hs: {=bool:?}, usb_hs_phy: {=bool:?}, els: {=bool:?}, pq: {=bool:?}, plu_lut: {=bool:?}, timer3: {=bool:?}, timer4: {=bool:?}, puf: {=bool:?}, pkc: {=bool:?}, scg: {=bool:?}, gdet: {=bool:?}, sm3: {=bool:?} }}",
            self.dma1(),
            self.enet(),
            self.u_sdhc(),
            self.flexio(),
            self.sai0(),
            self.sai1(),
            self.tro(),
            self.freqme(),
            self.trng(),
            self.flexcan0(),
            self.flexcan1(),
            self.usb_hs(),
            self.usb_hs_phy(),
            self.els(),
            self.pq(),
            self.plu_lut(),
            self.timer3(),
            self.timer4(),
            self.puf(),
            self.pkc(),
            self.scg(),
            self.gdet(),
            self.sm3()
        )
    }
}
#[doc = "AHB Clock Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrl3(pub u32);
impl Ahbclkctrl3 {
    #[doc = "Enables the clock for I3C0."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for I3C0."]
    #[inline(always)]
    pub const fn set_i3c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enables the clock for I3C1."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for I3C1."]
    #[inline(always)]
    pub const fn set_i3c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the clock for SINC."]
    #[must_use]
    #[inline(always)]
    pub const fn sinc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for SINC."]
    #[inline(always)]
    pub const fn set_sinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the clock for CoolFlux."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CoolFlux."]
    #[inline(always)]
    pub const fn set_coolflux(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables the clock for QDC0."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for QDC0."]
    #[inline(always)]
    pub const fn set_qdc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables the clock for QDC1."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for QDC1."]
    #[inline(always)]
    pub const fn set_qdc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables the clock for PWM0."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PWM0."]
    #[inline(always)]
    pub const fn set_pwm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables the clock for PWM1."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for PWM1."]
    #[inline(always)]
    pub const fn set_pwm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enables the clock for EVTG."]
    #[must_use]
    #[inline(always)]
    pub const fn evtg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EVTG."]
    #[inline(always)]
    pub const fn set_evtg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enables the clock for DAC1."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DAC1."]
    #[inline(always)]
    pub const fn set_dac1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables the clock for DAC2."]
    #[must_use]
    #[inline(always)]
    pub const fn dac2(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for DAC2."]
    #[inline(always)]
    pub const fn set_dac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Enables the clock for OPAMP."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp(&self, n: usize) -> bool {
        assert!(n < 3usize);
        let offs = 13usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for OPAMP."]
    #[inline(always)]
    pub const fn set_opamp(&mut self, n: usize, val: bool) {
        assert!(n < 3usize);
        let offs = 13usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Enables the clock for CMP2."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CMP2."]
    #[inline(always)]
    pub const fn set_cmp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Enables the clock for VREF."]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for VREF."]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Enables the clock for CoolFlux APB."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_apb(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for CoolFlux APB."]
    #[inline(always)]
    pub const fn set_coolflux_apb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enables the clock for NPU."]
    #[must_use]
    #[inline(always)]
    pub const fn npu(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for NPU."]
    #[inline(always)]
    pub const fn set_npu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Enables the clock for TSI."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for TSI."]
    #[inline(always)]
    pub const fn set_tsi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Enables the clock for EWM."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EWM."]
    #[inline(always)]
    pub const fn set_ewm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Enables the clock for EIM."]
    #[must_use]
    #[inline(always)]
    pub const fn eim(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for EIM."]
    #[inline(always)]
    pub const fn set_eim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Enables the clock for ERM."]
    #[must_use]
    #[inline(always)]
    pub const fn erm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for ERM."]
    #[inline(always)]
    pub const fn set_erm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Enables the clock for INTM."]
    #[must_use]
    #[inline(always)]
    pub const fn intm(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for INTM."]
    #[inline(always)]
    pub const fn set_intm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Enables the clock for Semaphore."]
    #[must_use]
    #[inline(always)]
    pub const fn sema42(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clock for Semaphore."]
    #[inline(always)]
    pub const fn set_sema42(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for Ahbclkctrl3 {
    #[inline(always)]
    fn default() -> Ahbclkctrl3 {
        Ahbclkctrl3(0)
    }
}
impl core::fmt::Debug for Ahbclkctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrl3")
            .field("i3c0", &self.i3c0())
            .field("i3c1", &self.i3c1())
            .field("sinc", &self.sinc())
            .field("coolflux", &self.coolflux())
            .field("qdc0", &self.qdc0())
            .field("qdc1", &self.qdc1())
            .field("pwm0", &self.pwm0())
            .field("pwm1", &self.pwm1())
            .field("evtg", &self.evtg())
            .field("dac1", &self.dac1())
            .field("dac2", &self.dac2())
            .field("opamp[0]", &self.opamp(0usize))
            .field("opamp[1]", &self.opamp(1usize))
            .field("opamp[2]", &self.opamp(2usize))
            .field("cmp2", &self.cmp2())
            .field("vref", &self.vref())
            .field("coolflux_apb", &self.coolflux_apb())
            .field("npu", &self.npu())
            .field("tsi", &self.tsi())
            .field("ewm", &self.ewm())
            .field("eim", &self.eim())
            .field("erm", &self.erm())
            .field("intm", &self.intm())
            .field("sema42", &self.sema42())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkctrl3 {{ i3c0: {=bool:?}, i3c1: {=bool:?}, sinc: {=bool:?}, coolflux: {=bool:?}, qdc0: {=bool:?}, qdc1: {=bool:?}, pwm0: {=bool:?}, pwm1: {=bool:?}, evtg: {=bool:?}, dac1: {=bool:?}, dac2: {=bool:?}, opamp[0]: {=bool:?}, opamp[1]: {=bool:?}, opamp[2]: {=bool:?}, cmp2: {=bool:?}, vref: {=bool:?}, coolflux_apb: {=bool:?}, npu: {=bool:?}, tsi: {=bool:?}, ewm: {=bool:?}, eim: {=bool:?}, erm: {=bool:?}, intm: {=bool:?}, sema42: {=bool:?} }}",
            self.i3c0(),
            self.i3c1(),
            self.sinc(),
            self.coolflux(),
            self.qdc0(),
            self.qdc1(),
            self.pwm0(),
            self.pwm1(),
            self.evtg(),
            self.dac1(),
            self.dac2(),
            self.opamp(0usize),
            self.opamp(1usize),
            self.opamp(2usize),
            self.cmp2(),
            self.vref(),
            self.coolflux_apb(),
            self.npu(),
            self.tsi(),
            self.ewm(),
            self.eim(),
            self.erm(),
            self.intm(),
            self.sema42()
        )
    }
}
#[doc = "AHB Clock Control Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrlclr(pub u32);
impl Ahbclkctrlclr {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ahbclkctrlclr {
    #[inline(always)]
    fn default() -> Ahbclkctrlclr {
        Ahbclkctrlclr(0)
    }
}
impl core::fmt::Debug for Ahbclkctrlclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrlclr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrlclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ahbclkctrlclr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "AHB Clock Control Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkctrlset(pub u32);
impl Ahbclkctrlset {
    #[doc = "Data array value."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Ahbclkctrlset {
    #[inline(always)]
    fn default() -> Ahbclkctrlset {
        Ahbclkctrlset(0)
    }
}
impl core::fmt::Debug for Ahbclkctrlset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkctrlset")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkctrlset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ahbclkctrlset {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "System Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbclkdiv(pub u32);
impl Ahbclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> AhbclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        AhbclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: AhbclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ahbclkdiv {
    #[inline(always)]
    fn default() -> Ahbclkdiv {
        Ahbclkdiv(0)
    }
}
impl core::fmt::Debug for Ahbclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbclkdiv")
            .field("div", &self.div())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbclkdiv {{ div: {=u8:?}, unstab: {:?} }}",
            self.div(),
            self.unstab()
        )
    }
}
#[doc = "AHB Matrix Priority Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ahbmatprio(pub u32);
impl Ahbmatprio {
    #[doc = "CPU0 C-AHB bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu0_cbus(&self) -> PriCpu0Cbus {
        let val = (self.0 >> 0usize) & 0x03;
        PriCpu0Cbus::from_bits(val as u8)
    }
    #[doc = "CPU0 C-AHB bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_cpu0_cbus(&mut self, val: PriCpu0Cbus) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 S-AHB bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu0_sbus(&self) -> PriCpu0Sbus {
        let val = (self.0 >> 2usize) & 0x03;
        PriCpu0Sbus::from_bits(val as u8)
    }
    #[doc = "CPU0 S-AHB bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_cpu0_sbus(&mut self, val: PriCpu0Sbus) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU1 S-AHB/SmartDMA-D bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu1_sbus_smart_dma_d(&self) -> PriCpu1SbusSmartDmaD {
        let val = (self.0 >> 4usize) & 0x03;
        PriCpu1SbusSmartDmaD::from_bits(val as u8)
    }
    #[doc = "CPU1 S-AHB/SmartDMA-D bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_cpu1_sbus_smart_dma_d(&mut self, val: PriCpu1SbusSmartDmaD) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU1 C-AHB/SmartDMA-I bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_cpu1_cbus_smart_dma_i(&self) -> PriCpu1CbusSmartDmaI {
        let val = (self.0 >> 6usize) & 0x03;
        PriCpu1CbusSmartDmaI::from_bits(val as u8)
    }
    #[doc = "CPU1 C-AHB/SmartDMA-I bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_cpu1_cbus_smart_dma_i(&mut self, val: PriCpu1CbusSmartDmaI) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DMA0 controller bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0(&self) -> AhbmatprioDma0 {
        let val = (self.0 >> 8usize) & 0x03;
        AhbmatprioDma0::from_bits(val as u8)
    }
    #[doc = "DMA0 controller bus master priority level."]
    #[inline(always)]
    pub const fn set_dma0(&mut self, val: AhbmatprioDma0) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "DMA1 controller bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1(&self) -> AhbmatprioDma1 {
        let val = (self.0 >> 10usize) & 0x03;
        AhbmatprioDma1::from_bits(val as u8)
    }
    #[doc = "DMA1 controller bus master priority level."]
    #[inline(always)]
    pub const fn set_dma1(&mut self, val: AhbmatprioDma1) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "PKC and ELS bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_pkc_els(&self) -> PriPkcEls {
        let val = (self.0 >> 12usize) & 0x03;
        PriPkcEls::from_bits(val as u8)
    }
    #[doc = "PKC and ELS bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_pkc_els(&mut self, val: PriPkcEls) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "NPU O bus and Powerquad bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_npu_pq(&self) -> PriNpuPq {
        let val = (self.0 >> 14usize) & 0x03;
        PriNpuPq::from_bits(val as u8)
    }
    #[doc = "NPU O bus and Powerquad bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_npu_pq(&mut self, val: PriNpuPq) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CoolFlux I bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_coolflux_i(&self) -> PriCoolfluxI {
        let val = (self.0 >> 16usize) & 0x03;
        PriCoolfluxI::from_bits(val as u8)
    }
    #[doc = "CoolFlux I bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_coolflux_i(&mut self, val: PriCoolfluxI) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "CoolFlux X bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_coolflux_x(&self) -> PriCoolfluxX {
        let val = (self.0 >> 18usize) & 0x03;
        PriCoolfluxX::from_bits(val as u8)
    }
    #[doc = "CoolFlux X bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_coolflux_x(&mut self, val: PriCoolfluxX) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
    #[doc = "CoolFlux Y bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_coolflux_y_espi(&self) -> PriCoolfluxYEspi {
        let val = (self.0 >> 20usize) & 0x03;
        PriCoolfluxYEspi::from_bits(val as u8)
    }
    #[doc = "CoolFlux Y bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_coolflux_y_espi(&mut self, val: PriCoolfluxYEspi) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "NPU D bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_npu_d(&self) -> PriNpuD {
        let val = (self.0 >> 22usize) & 0x03;
        PriNpuD::from_bits(val as u8)
    }
    #[doc = "NPU D bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_npu_d(&mut self, val: PriNpuD) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
    }
    #[doc = "USB-FS and ENET bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usb_fs_enet(&self) -> PriUsbFsEnet {
        let val = (self.0 >> 24usize) & 0x03;
        PriUsbFsEnet::from_bits(val as u8)
    }
    #[doc = "USB-FS and ENET bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_usb_fs_enet(&mut self, val: PriUsbFsEnet) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "USB-HS bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usb_hs(&self) -> PriUsbHs {
        let val = (self.0 >> 26usize) & 0x03;
        PriUsbHs::from_bits(val as u8)
    }
    #[doc = "USB-HS bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_usb_hs(&mut self, val: PriUsbHs) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "USDHC bus master priority level."]
    #[must_use]
    #[inline(always)]
    pub const fn pri_usdhc(&self) -> PriUsdhc {
        let val = (self.0 >> 28usize) & 0x03;
        PriUsdhc::from_bits(val as u8)
    }
    #[doc = "USDHC bus master priority level."]
    #[inline(always)]
    pub const fn set_pri_usdhc(&mut self, val: PriUsdhc) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for Ahbmatprio {
    #[inline(always)]
    fn default() -> Ahbmatprio {
        Ahbmatprio(0)
    }
}
impl core::fmt::Debug for Ahbmatprio {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ahbmatprio")
            .field("pri_cpu0_cbus", &self.pri_cpu0_cbus())
            .field("pri_cpu0_sbus", &self.pri_cpu0_sbus())
            .field(
                "pri_cpu1_sbus_smart_dma_d",
                &self.pri_cpu1_sbus_smart_dma_d(),
            )
            .field(
                "pri_cpu1_cbus_smart_dma_i",
                &self.pri_cpu1_cbus_smart_dma_i(),
            )
            .field("dma0", &self.dma0())
            .field("dma1", &self.dma1())
            .field("pri_pkc_els", &self.pri_pkc_els())
            .field("pri_npu_pq", &self.pri_npu_pq())
            .field("pri_coolflux_i", &self.pri_coolflux_i())
            .field("pri_coolflux_x", &self.pri_coolflux_x())
            .field("pri_coolflux_y_espi", &self.pri_coolflux_y_espi())
            .field("pri_npu_d", &self.pri_npu_d())
            .field("pri_usb_fs_enet", &self.pri_usb_fs_enet())
            .field("pri_usb_hs", &self.pri_usb_hs())
            .field("pri_usdhc", &self.pri_usdhc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ahbmatprio {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ahbmatprio {{ pri_cpu0_cbus: {:?}, pri_cpu0_sbus: {:?}, pri_cpu1_sbus_smart_dma_d: {:?}, pri_cpu1_cbus_smart_dma_i: {:?}, dma0: {:?}, dma1: {:?}, pri_pkc_els: {:?}, pri_npu_pq: {:?}, pri_coolflux_i: {:?}, pri_coolflux_x: {:?}, pri_coolflux_y_espi: {:?}, pri_npu_d: {:?}, pri_usb_fs_enet: {:?}, pri_usb_hs: {:?}, pri_usdhc: {:?} }}",
            self.pri_cpu0_cbus(),
            self.pri_cpu0_sbus(),
            self.pri_cpu1_sbus_smart_dma_d(),
            self.pri_cpu1_cbus_smart_dma_i(),
            self.dma0(),
            self.dma1(),
            self.pri_pkc_els(),
            self.pri_npu_pq(),
            self.pri_coolflux_i(),
            self.pri_coolflux_x(),
            self.pri_coolflux_y_espi(),
            self.pri_npu_d(),
            self.pri_usb_fs_enet(),
            self.pri_usb_hs(),
            self.pri_usdhc()
        )
    }
}
#[doc = "Control Automatic Clock Gating."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverride(pub u32);
impl Autoclkgateoverride {
    #[doc = "Controls automatic clock gating for the RAMB Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_ctrl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMB Controller."]
    #[inline(always)]
    pub const fn set_ramb_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Controls automatic clock gating for the RAMC Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramc_ctrl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMC Controller."]
    #[inline(always)]
    pub const fn set_ramc_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramd_ctrl(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    #[inline(always)]
    pub const fn set_ramd_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn rame_ctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMD Controller."]
    #[inline(always)]
    pub const fn set_rame_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Controls automatic clock gating for the RAMF Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramf_ctrl(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMF Controller."]
    #[inline(always)]
    pub const fn set_ramf_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramg_ctrl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller."]
    #[inline(always)]
    pub const fn set_ramg_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramh_ctrl(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating for the RAMG Controller."]
    #[inline(always)]
    pub const fn set_ramh_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Autoclkgateoverride {
    #[inline(always)]
    fn default() -> Autoclkgateoverride {
        Autoclkgateoverride(0)
    }
}
impl core::fmt::Debug for Autoclkgateoverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoclkgateoverride")
            .field("ramb_ctrl", &self.ramb_ctrl())
            .field("ramc_ctrl", &self.ramc_ctrl())
            .field("ramd_ctrl", &self.ramd_ctrl())
            .field("rame_ctrl", &self.rame_ctrl())
            .field("ramf_ctrl", &self.ramf_ctrl())
            .field("ramg_ctrl", &self.ramg_ctrl())
            .field("ramh_ctrl", &self.ramh_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverride {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverride {{ ramb_ctrl: {=bool:?}, ramc_ctrl: {=bool:?}, ramd_ctrl: {=bool:?}, rame_ctrl: {=bool:?}, ramf_ctrl: {=bool:?}, ramg_ctrl: {=bool:?}, ramh_ctrl: {=bool:?} }}",
            self.ramb_ctrl(),
            self.ramc_ctrl(),
            self.ramd_ctrl(),
            self.rame_ctrl(),
            self.ramf_ctrl(),
            self.ramg_ctrl(),
            self.ramh_ctrl()
        )
    }
}
#[doc = "Control Automatic Clock Gating C."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Autoclkgateoverridec(pub u32);
impl Autoclkgateoverridec {
    #[doc = "Controls automatic clock gating of the RAMX controller."]
    #[must_use]
    #[inline(always)]
    pub const fn ramx(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating of the RAMX controller."]
    #[inline(always)]
    pub const fn set_ramx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Controls automatic clock gating of the RAMA controller."]
    #[must_use]
    #[inline(always)]
    pub const fn rama(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Controls automatic clock gating of the RAMA controller."]
    #[inline(always)]
    pub const fn set_rama(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Autoclkgateoverridec {
    #[inline(always)]
    fn default() -> Autoclkgateoverridec {
        Autoclkgateoverridec(0)
    }
}
impl core::fmt::Debug for Autoclkgateoverridec {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Autoclkgateoverridec")
            .field("ramx", &self.ramx())
            .field("rama", &self.rama())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Autoclkgateoverridec {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Autoclkgateoverridec {{ ramx: {=bool:?}, rama: {=bool:?} }}",
            self.ramx(),
            self.rama()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[31:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeLsb(pub u32);
impl BinaryCodeLsb {
    #[doc = "Binary code \\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Binary code \\[31:0\\]."]
    #[inline(always)]
    pub const fn set_code_bin_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BinaryCodeLsb {
    #[inline(always)]
    fn default() -> BinaryCodeLsb {
        BinaryCodeLsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeLsb")
            .field("code_bin_31_0", &self.code_bin_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeLsb {{ code_bin_31_0: {=u32:?} }}",
            self.code_bin_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Binary Code \\[41:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BinaryCodeMsb(pub u32);
impl BinaryCodeMsb {
    #[doc = "Binary code \\[41:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_bin_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Binary code \\[41:32\\]."]
    #[inline(always)]
    pub const fn set_code_bin_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BinaryCodeMsb {
    #[inline(always)]
    fn default() -> BinaryCodeMsb {
        BinaryCodeMsb(0)
    }
}
impl core::fmt::Debug for BinaryCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BinaryCodeMsb")
            .field("code_bin_41_32", &self.code_bin_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BinaryCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BinaryCodeMsb {{ code_bin_41_32: {=u16:?} }}",
            self.code_bin_41_32()
        )
    }
}
#[doc = "CLKOUT FRG Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkoutFrgctrl(pub u32);
impl ClkoutFrgctrl {
    #[doc = "Divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Numerator value."]
    #[must_use]
    #[inline(always)]
    pub const fn mult(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Numerator value."]
    #[inline(always)]
    pub const fn set_mult(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ClkoutFrgctrl {
    #[inline(always)]
    fn default() -> ClkoutFrgctrl {
        ClkoutFrgctrl(0)
    }
}
impl core::fmt::Debug for ClkoutFrgctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkoutFrgctrl")
            .field("div", &self.div())
            .field("mult", &self.mult())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkoutFrgctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClkoutFrgctrl {{ div: {=u8:?}, mult: {=u8:?} }}",
            self.div(),
            self.mult()
        )
    }
}
#[doc = "CLKOUT Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutdiv(pub u32);
impl Clkoutdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> ClkoutdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        ClkoutdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: ClkoutdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> ClkoutdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        ClkoutdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: ClkoutdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> ClkoutdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        ClkoutdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: ClkoutdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Clkoutdiv {
    #[inline(always)]
    fn default() -> Clkoutdiv {
        Clkoutdiv(0)
    }
}
impl core::fmt::Debug for Clkoutdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkoutdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CLKOUT Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkoutsel(pub u32);
impl Clkoutsel {
    #[doc = "Selects the CLKOUT clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> ClkoutselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        ClkoutselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CLKOUT clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: ClkoutselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Clkoutsel {
    #[inline(always)]
    fn default() -> Clkoutsel {
        Clkoutsel(0)
    }
}
impl core::fmt::Debug for Clkoutsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkoutsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkoutsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkoutsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Clock Configuration Unlock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkunlock(pub u32);
impl Clkunlock {
    #[doc = "Controls clock configuration registers access (for example, xxxDIV, xxxSEL)."]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> Unlock {
        let val = (self.0 >> 0usize) & 0x01;
        Unlock::from_bits(val as u8)
    }
    #[doc = "Controls clock configuration registers access (for example, xxxDIV, xxxSEL)."]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: Unlock) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Clkunlock {
    #[inline(always)]
    fn default() -> Clkunlock {
        Clkunlock(0)
    }
}
impl core::fmt::Debug for Clkunlock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkunlock")
            .field("unlock", &self.unlock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkunlock {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Clkunlock {{ unlock: {:?} }}", self.unlock())
    }
}
#[doc = "Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClockCtrl(pub u32);
impl ClockCtrl {
    #[doc = "Enables the clk_in clock for the Frequency Measurement, USB HS and LPTMR0/1 modules."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin_ena_fm_usbh_lpt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the clk_in clock for the Frequency Measurement, USB HS and LPTMR0/1 modules."]
    #[inline(always)]
    pub const fn set_clkin_ena_fm_usbh_lpt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables the FRO_1MHz clock for RTC module and for UTICK."]
    #[must_use]
    #[inline(always)]
    pub const fn fro1mhz_ena(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the FRO_1MHz clock for RTC module and for UTICK."]
    #[inline(always)]
    pub const fn set_fro1mhz_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the FRO_12MHz clock for the Flash, LPTMR0/1, and Frequency Measurement modules."]
    #[must_use]
    #[inline(always)]
    pub const fn fro12mhz_ena(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the FRO_12MHz clock for the Flash, LPTMR0/1, and Frequency Measurement modules."]
    #[inline(always)]
    pub const fn set_fro12mhz_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enables FRO HF clock for the Frequency Measure module."]
    #[must_use]
    #[inline(always)]
    pub const fn fro_hf_ena(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables FRO HF clock for the Frequency Measure module."]
    #[inline(always)]
    pub const fn set_fro_hf_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enables clk_in clock for MICFIL, CAN0/1, I3C0/1, SAI0/1, clkout."]
    #[must_use]
    #[inline(always)]
    pub const fn clkin_ena(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enables clk_in clock for MICFIL, CAN0/1, I3C0/1, SAI0/1, clkout."]
    #[inline(always)]
    pub const fn set_clkin_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enables FRO_1MHz clock for clock muxing in clock gen."]
    #[must_use]
    #[inline(always)]
    pub const fn fro1mhz_clk_ena(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enables FRO_1MHz clock for clock muxing in clock gen."]
    #[inline(always)]
    pub const fn set_fro1mhz_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enables clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[must_use]
    #[inline(always)]
    pub const fn plu_deglitch_clk_ena(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables clocks FRO_1MHz and FRO_12MHz for PLU deglitching."]
    #[inline(always)]
    pub const fn set_plu_deglitch_clk_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for ClockCtrl {
    #[inline(always)]
    fn default() -> ClockCtrl {
        ClockCtrl(0)
    }
}
impl core::fmt::Debug for ClockCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClockCtrl")
            .field("clkin_ena_fm_usbh_lpt", &self.clkin_ena_fm_usbh_lpt())
            .field("fro1mhz_ena", &self.fro1mhz_ena())
            .field("fro12mhz_ena", &self.fro12mhz_ena())
            .field("fro_hf_ena", &self.fro_hf_ena())
            .field("clkin_ena", &self.clkin_ena())
            .field("fro1mhz_clk_ena", &self.fro1mhz_clk_ena())
            .field("plu_deglitch_clk_ena", &self.plu_deglitch_clk_ena())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClockCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ClockCtrl {{ clkin_ena_fm_usbh_lpt: {=bool:?}, fro1mhz_ena: {=bool:?}, fro12mhz_ena: {=bool:?}, fro_hf_ena: {=bool:?}, clkin_ena: {=bool:?}, fro1mhz_clk_ena: {=bool:?}, plu_deglitch_clk_ena: {=bool:?} }}",
            self.clkin_ena_fm_usbh_lpt(),
            self.fro1mhz_ena(),
            self.fro12mhz_ena(),
            self.fro_hf_ena(),
            self.clkin_ena(),
            self.fro1mhz_clk_ena(),
            self.plu_deglitch_clk_ena()
        )
    }
}
#[doc = "CMP0 Round Robin Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp0rrclksel(pub u32);
impl Cmp0rrclksel {
    #[doc = "Selects the CMP0 round robin clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Cmp0rrclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Cmp0rrclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP0 round robin clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Cmp0rrclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp0rrclksel {
    #[inline(always)]
    fn default() -> Cmp0rrclksel {
        Cmp0rrclksel(0)
    }
}
impl core::fmt::Debug for Cmp0rrclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp0rrclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp0rrclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp0rrclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP1 Round Robin Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp1rrclksel(pub u32);
impl Cmp1rrclksel {
    #[doc = "Selects the CMP1 round robin clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Cmp1rrclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Cmp1rrclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP1 round robin clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Cmp1rrclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp1rrclksel {
    #[inline(always)]
    fn default() -> Cmp1rrclksel {
        Cmp1rrclksel(0)
    }
}
impl core::fmt::Debug for Cmp1rrclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp1rrclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp1rrclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp1rrclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP2 Round Robin Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp2rrclksel(pub u32);
impl Cmp2rrclksel {
    #[doc = "Selects the CMP2 round robin clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Cmp2rrclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Cmp2rrclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP2 round robin clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Cmp2rrclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmp2rrclksel {
    #[inline(always)]
    fn default() -> Cmp2rrclksel {
        Cmp2rrclksel(0)
    }
}
impl core::fmt::Debug for Cmp2rrclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmp2rrclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmp2rrclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmp2rrclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpfclkdiv(pub u32);
impl Cmpfclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> CmpfclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        CmpfclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: CmpfclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> CmpfclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        CmpfclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: CmpfclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> CmpfclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        CmpfclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: CmpfclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmpfclkdiv {
    #[inline(always)]
    fn default() -> Cmpfclkdiv {
        Cmpfclkdiv(0)
    }
}
impl core::fmt::Debug for Cmpfclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpfclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpfclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmpfclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CMP Function Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpfclksel(pub u32);
impl Cmpfclksel {
    #[doc = "Selects the CMP function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> CmpfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        CmpfclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CMP function clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: CmpfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Cmpfclksel {
    #[inline(always)]
    fn default() -> Cmpfclksel {
        Cmpfclksel(0)
    }
}
impl core::fmt::Debug for Cmpfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmpfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CMP Round Robin Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmprrclkdiv(pub u32);
impl Cmprrclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> CmprrclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        CmprrclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: CmprrclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> CmprrclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        CmprrclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: CmprrclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> CmprrclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        CmprrclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: CmprrclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmprrclkdiv {
    #[inline(always)]
    fn default() -> Cmprrclkdiv {
        Cmprrclkdiv(0)
    }
}
impl core::fmt::Debug for Cmprrclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmprrclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmprrclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmprrclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Coprocessor Boot Address."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpboot(pub u32);
impl Cpboot {
    #[doc = "Coprocessor Boot VTOR Address \\[31:7\\] for CPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn cpboot(&self) -> u32 {
        let val = (self.0 >> 7usize) & 0x01ff_ffff;
        val as u32
    }
    #[doc = "Coprocessor Boot VTOR Address \\[31:7\\] for CPU1."]
    #[inline(always)]
    pub const fn set_cpboot(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
    }
}
impl Default for Cpboot {
    #[inline(always)]
    fn default() -> Cpboot {
        Cpboot(0)
    }
}
impl core::fmt::Debug for Cpboot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpboot")
            .field("cpboot", &self.cpboot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpboot {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cpboot {{ cpboot: {=u32:?} }}", self.cpboot())
    }
}
#[doc = "Non-Secure CPU0 System Tick Calibration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0nstckcal(pub u32);
impl Cpu0nstckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> Cpu0nstckcalSkew {
        let val = (self.0 >> 24usize) & 0x01;
        Cpu0nstckcalSkew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: Cpu0nstckcalSkew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> Cpu0nstckcalNoref {
        let val = (self.0 >> 25usize) & 0x01;
        Cpu0nstckcalNoref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: Cpu0nstckcalNoref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0nstckcal {
    #[inline(always)]
    fn default() -> Cpu0nstckcal {
        Cpu0nstckcal(0)
    }
}
impl core::fmt::Debug for Cpu0nstckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0nstckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0nstckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0nstckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "Secure CPU0 System Tick Calibration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0stckcal(pub u32);
impl Cpu0stckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> Cpu0stckcalSkew {
        let val = (self.0 >> 24usize) & 0x01;
        Cpu0stckcalSkew::from_bits(val as u8)
    }
    #[doc = "Whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: Cpu0stckcalSkew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> Cpu0stckcalNoref {
        let val = (self.0 >> 25usize) & 0x01;
        Cpu0stckcalNoref::from_bits(val as u8)
    }
    #[doc = "Whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: Cpu0stckcalNoref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu0stckcal {
    #[inline(always)]
    fn default() -> Cpu0stckcal {
        Cpu0stckcal(0)
    }
}
impl core::fmt::Debug for Cpu0stckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu0stckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu0stckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu0stckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "System tick calibration for CPU1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1stckcal(pub u32);
impl Cpu1stckcal {
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[must_use]
    #[inline(always)]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reload value for 10 ms (100 Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub const fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[must_use]
    #[inline(always)]
    pub const fn skew(&self) -> Cpu1stckcalSkew {
        let val = (self.0 >> 24usize) & 0x01;
        Cpu1stckcalSkew::from_bits(val as u8)
    }
    #[doc = "Indicates whether the TENMS value is exact."]
    #[inline(always)]
    pub const fn set_skew(&mut self, val: Cpu1stckcalSkew) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[must_use]
    #[inline(always)]
    pub const fn noref(&self) -> Cpu1stckcalNoref {
        let val = (self.0 >> 25usize) & 0x01;
        Cpu1stckcalNoref::from_bits(val as u8)
    }
    #[doc = "Indicates whether the device provides a reference clock to the processor."]
    #[inline(always)]
    pub const fn set_noref(&mut self, val: Cpu1stckcalNoref) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Cpu1stckcal {
    #[inline(always)]
    fn default() -> Cpu1stckcal {
        Cpu1stckcal(0)
    }
}
impl core::fmt::Debug for Cpu1stckcal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpu1stckcal")
            .field("tenms", &self.tenms())
            .field("skew", &self.skew())
            .field("noref", &self.noref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpu1stckcal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpu1stckcal {{ tenms: {=u32:?}, skew: {:?}, noref: {:?} }}",
            self.tenms(),
            self.skew(),
            self.noref()
        )
    }
}
#[doc = "CPU Control for Multiple Processors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpuctrl(pub u32);
impl Cpuctrl {
    #[doc = "Enables the CPU1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1clken(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the CPU1 clock."]
    #[inline(always)]
    pub const fn set_cpu1clken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "CPU1 reset."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1rsten(&self) -> Cpu1rsten {
        let val = (self.0 >> 5usize) & 0x01;
        Cpu1rsten::from_bits(val as u8)
    }
    #[doc = "CPU1 reset."]
    #[inline(always)]
    pub const fn set_cpu1rsten(&mut self, val: Cpu1rsten) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Write Protect."]
    #[must_use]
    #[inline(always)]
    pub const fn prot(&self) -> Prot {
        let val = (self.0 >> 16usize) & 0xffff;
        Prot::from_bits(val as u16)
    }
    #[doc = "Write Protect."]
    #[inline(always)]
    pub const fn set_prot(&mut self, val: Prot) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cpuctrl {
    #[inline(always)]
    fn default() -> Cpuctrl {
        Cpuctrl(0)
    }
}
impl core::fmt::Debug for Cpuctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpuctrl")
            .field("cpu1clken", &self.cpu1clken())
            .field("cpu1rsten", &self.cpu1rsten())
            .field("prot", &self.prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpuctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpuctrl {{ cpu1clken: {=bool:?}, cpu1rsten: {:?}, prot: {:?} }}",
            self.cpu1clken(),
            self.cpu1rsten(),
            self.prot()
        )
    }
}
#[doc = "CPU Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpustat(pub u32);
impl Cpustat {
    #[doc = "CPU0 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0sleeping(&self) -> Cpu0sleeping {
        let val = (self.0 >> 0usize) & 0x01;
        Cpu0sleeping::from_bits(val as u8)
    }
    #[doc = "CPU0 sleeping state."]
    #[inline(always)]
    pub const fn set_cpu0sleeping(&mut self, val: Cpu0sleeping) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "CPU1 sleeping state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1sleeping(&self) -> Cpu1sleeping {
        let val = (self.0 >> 1usize) & 0x01;
        Cpu1sleeping::from_bits(val as u8)
    }
    #[doc = "CPU1 sleeping state."]
    #[inline(always)]
    pub const fn set_cpu1sleeping(&mut self, val: Cpu1sleeping) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU0 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0lockup(&self) -> Cpu0lockup {
        let val = (self.0 >> 2usize) & 0x01;
        Cpu0lockup::from_bits(val as u8)
    }
    #[doc = "CPU0 lockup state."]
    #[inline(always)]
    pub const fn set_cpu0lockup(&mut self, val: Cpu0lockup) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CPU1 lockup state."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1lockup(&self) -> Cpu1lockup {
        let val = (self.0 >> 3usize) & 0x01;
        Cpu1lockup::from_bits(val as u8)
    }
    #[doc = "CPU1 lockup state."]
    #[inline(always)]
    pub const fn set_cpu1lockup(&mut self, val: Cpu1lockup) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Cpustat {
    #[inline(always)]
    fn default() -> Cpustat {
        Cpustat(0)
    }
}
impl core::fmt::Debug for Cpustat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cpustat")
            .field("cpu0sleeping", &self.cpu0sleeping())
            .field("cpu1sleeping", &self.cpu1sleeping())
            .field("cpu0lockup", &self.cpu0lockup())
            .field("cpu1lockup", &self.cpu1lockup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cpustat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cpustat {{ cpu0sleeping: {:?}, cpu1sleeping: {:?}, cpu0lockup: {:?}, cpu1lockup: {:?} }}",
            self.cpu0sleeping(),
            self.cpu1sleeping(),
            self.cpu0lockup(),
            self.cpu1lockup()
        )
    }
}
#[doc = "CTimer Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclkdiv(pub u32);
impl Ctimerclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> CtimerclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        CtimerclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: CtimerclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> CtimerclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        CtimerclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: CtimerclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctimerclkdiv {
    #[inline(always)]
    fn default() -> Ctimerclkdiv {
        Ctimerclkdiv(0)
    }
}
impl core::fmt::Debug for Ctimerclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerclkdiv {{ div: {=u8:?}, reset: {=bool:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CTIMER Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerclksel(pub u32);
impl Ctimerclksel {
    #[doc = "Selects the CTIMER clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> CtimerclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        CtimerclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the CTIMER clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: CtimerclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Ctimerclksel {
    #[inline(always)]
    fn default() -> Ctimerclksel {
        Ctimerclksel(0)
    }
}
impl core::fmt::Debug for Ctimerclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ctimerclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CTIMER Global Start Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimerglobalstarten(pub u32);
impl Ctimerglobalstarten {
    #[doc = "Enables the CTIMER function clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ctimer_clk_en(&self, n: usize) -> bool {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables the CTIMER function clock."]
    #[inline(always)]
    pub const fn set_ctimer_clk_en(&mut self, n: usize, val: bool) {
        assert!(n < 5usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Ctimerglobalstarten {
    #[inline(always)]
    fn default() -> Ctimerglobalstarten {
        Ctimerglobalstarten(0)
    }
}
impl core::fmt::Debug for Ctimerglobalstarten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctimerglobalstarten")
            .field("ctimer_clk_en[0]", &self.ctimer_clk_en(0usize))
            .field("ctimer_clk_en[1]", &self.ctimer_clk_en(1usize))
            .field("ctimer_clk_en[2]", &self.ctimer_clk_en(2usize))
            .field("ctimer_clk_en[3]", &self.ctimer_clk_en(3usize))
            .field("ctimer_clk_en[4]", &self.ctimer_clk_en(4usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctimerglobalstarten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Ctimerglobalstarten {{ ctimer_clk_en[0]: {=bool:?}, ctimer_clk_en[1]: {=bool:?}, ctimer_clk_en[2]: {=bool:?}, ctimer_clk_en[3]: {=bool:?}, ctimer_clk_en[4]: {=bool:?} }}",
            self.ctimer_clk_en(0usize),
            self.ctimer_clk_en(1usize),
            self.ctimer_clk_en(2usize),
            self.ctimer_clk_en(3usize),
            self.ctimer_clk_en(4usize)
        )
    }
}
#[doc = "DAC functional clock divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacclkdiv(pub u32);
impl Dacclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> DacclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        DacclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: DacclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> DacclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        DacclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: DacclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> DacclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        DacclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: DacclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dacclkdiv {
    #[inline(always)]
    fn default() -> Dacclkdiv {
        Dacclkdiv(0)
    }
}
impl core::fmt::Debug for Dacclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dacclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dacclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dacclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "DAC Functional Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dacclksel(pub u32);
impl Dacclksel {
    #[doc = "Selects the DAC clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> DacclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        DacclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the DAC clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: DacclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Dacclksel {
    #[inline(always)]
    fn default() -> Dacclksel {
        Dacclksel(0)
    }
}
impl core::fmt::Debug for Dacclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dacclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dacclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dacclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Debug Authentication BEACON."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugAuthBeacon(pub u32);
impl DebugAuthBeacon {
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[must_use]
    #[inline(always)]
    pub const fn beacon(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Sets by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to the application code."]
    #[inline(always)]
    pub const fn set_beacon(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DebugAuthBeacon {
    #[inline(always)]
    fn default() -> DebugAuthBeacon {
        DebugAuthBeacon(0)
    }
}
impl core::fmt::Debug for DebugAuthBeacon {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugAuthBeacon")
            .field("beacon", &self.beacon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugAuthBeacon {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugAuthBeacon {{ beacon: {=u32:?} }}", self.beacon())
    }
}
#[doc = "Cortex Debug Features Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeatures(pub u32);
impl DebugFeatures {
    #[doc = "CPU0 invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> DebugFeaturesCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        DebugFeaturesCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: DebugFeaturesCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> DebugFeaturesCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        DebugFeaturesCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: DebugFeaturesCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 secure privileged invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> DebugFeaturesCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        DebugFeaturesCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: DebugFeaturesCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 secure privileged non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> DebugFeaturesCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        DebugFeaturesCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: DebugFeaturesCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CPU1 invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> DebugFeaturesCpu1Dbgen {
        let val = (self.0 >> 8usize) & 0x03;
        DebugFeaturesCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: DebugFeaturesCpu1Dbgen) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU1 non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> DebugFeaturesCpu1Niden {
        let val = (self.0 >> 10usize) & 0x03;
        DebugFeaturesCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: DebugFeaturesCpu1Niden) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "DSP invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_dbgden(&self) -> DspDbgden {
        let val = (self.0 >> 12usize) & 0x03;
        DspDbgden::from_bits(val as u8)
    }
    #[doc = "DSP invasive debug control."]
    #[inline(always)]
    pub const fn set_dsp_dbgden(&mut self, val: DspDbgden) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for DebugFeatures {
    #[inline(always)]
    fn default() -> DebugFeatures {
        DebugFeatures(0)
    }
}
impl core::fmt::Debug for DebugFeatures {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeatures")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .field("cpu0_spiden", &self.cpu0_spiden())
            .field("cpu0_spniden", &self.cpu0_spniden())
            .field("cpu1_dbgen", &self.cpu1_dbgen())
            .field("cpu1_niden", &self.cpu1_niden())
            .field("dsp_dbgden", &self.dsp_dbgden())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeatures {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeatures {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?}, cpu1_dbgen: {:?}, cpu1_niden: {:?}, dsp_dbgden: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden(),
            self.cpu1_dbgen(),
            self.cpu1_niden(),
            self.dsp_dbgden()
        )
    }
}
#[doc = "Cortex Debug Features Control (Duplicate)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugFeaturesDp(pub u32);
impl DebugFeaturesDp {
    #[doc = "CPU0 invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_dbgen(&self) -> DebugFeaturesDpCpu0Dbgen {
        let val = (self.0 >> 0usize) & 0x03;
        DebugFeaturesDpCpu0Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU0 invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_dbgen(&mut self, val: DebugFeaturesDpCpu0Dbgen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_niden(&self) -> DebugFeaturesDpCpu0Niden {
        let val = (self.0 >> 2usize) & 0x03;
        DebugFeaturesDpCpu0Niden::from_bits(val as u8)
    }
    #[doc = "CPU0 non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_niden(&mut self, val: DebugFeaturesDpCpu0Niden) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "CPU0 secure privileged invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spiden(&self) -> DebugFeaturesDpCpu0Spiden {
        let val = (self.0 >> 4usize) & 0x03;
        DebugFeaturesDpCpu0Spiden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_spiden(&mut self, val: DebugFeaturesDpCpu0Spiden) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "CPU0 secure privileged non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu0_spniden(&self) -> DebugFeaturesDpCpu0Spniden {
        let val = (self.0 >> 6usize) & 0x03;
        DebugFeaturesDpCpu0Spniden::from_bits(val as u8)
    }
    #[doc = "CPU0 secure privileged non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu0_spniden(&mut self, val: DebugFeaturesDpCpu0Spniden) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "CPU1 invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_dbgen(&self) -> DebugFeaturesDpCpu1Dbgen {
        let val = (self.0 >> 8usize) & 0x03;
        DebugFeaturesDpCpu1Dbgen::from_bits(val as u8)
    }
    #[doc = "CPU1 invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu1_dbgen(&mut self, val: DebugFeaturesDpCpu1Dbgen) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "CPU1 non-invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn cpu1_niden(&self) -> DebugFeaturesDpCpu1Niden {
        let val = (self.0 >> 10usize) & 0x03;
        DebugFeaturesDpCpu1Niden::from_bits(val as u8)
    }
    #[doc = "CPU1 non-invasive debug control."]
    #[inline(always)]
    pub const fn set_cpu1_niden(&mut self, val: DebugFeaturesDpCpu1Niden) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "DSP invasive debug control."]
    #[must_use]
    #[inline(always)]
    pub const fn dsp_dbgen(&self) -> DspDbgen {
        let val = (self.0 >> 12usize) & 0x03;
        DspDbgen::from_bits(val as u8)
    }
    #[doc = "DSP invasive debug control."]
    #[inline(always)]
    pub const fn set_dsp_dbgen(&mut self, val: DspDbgen) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for DebugFeaturesDp {
    #[inline(always)]
    fn default() -> DebugFeaturesDp {
        DebugFeaturesDp(0)
    }
}
impl core::fmt::Debug for DebugFeaturesDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugFeaturesDp")
            .field("cpu0_dbgen", &self.cpu0_dbgen())
            .field("cpu0_niden", &self.cpu0_niden())
            .field("cpu0_spiden", &self.cpu0_spiden())
            .field("cpu0_spniden", &self.cpu0_spniden())
            .field("cpu1_dbgen", &self.cpu1_dbgen())
            .field("cpu1_niden", &self.cpu1_niden())
            .field("dsp_dbgen", &self.dsp_dbgen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugFeaturesDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DebugFeaturesDp {{ cpu0_dbgen: {:?}, cpu0_niden: {:?}, cpu0_spiden: {:?}, cpu0_spniden: {:?}, cpu1_dbgen: {:?}, cpu1_niden: {:?}, dsp_dbgen: {:?} }}",
            self.cpu0_dbgen(),
            self.cpu0_niden(),
            self.cpu0_spiden(),
            self.cpu0_spniden(),
            self.cpu1_dbgen(),
            self.cpu1_niden(),
            self.dsp_dbgen()
        )
    }
}
#[doc = "Control Write Access to Security."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLockEn(pub u32);
impl DebugLockEn {
    #[doc = "Controls write access to the security registers."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_all(&self) -> LockAll {
        let val = (self.0 >> 0usize) & 0x0f;
        LockAll::from_bits(val as u8)
    }
    #[doc = "Controls write access to the security registers."]
    #[inline(always)]
    pub const fn set_lock_all(&mut self, val: LockAll) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for DebugLockEn {
    #[inline(always)]
    fn default() -> DebugLockEn {
        DebugLockEn(0)
    }
}
impl core::fmt::Debug for DebugLockEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugLockEn")
            .field("lock_all", &self.lock_all())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugLockEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DebugLockEn {{ lock_all: {:?} }}", self.lock_all())
    }
}
#[doc = "Device ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceId0(pub u32);
impl DeviceId0 {
    #[doc = "ROM revision."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rev_minor(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM revision."]
    #[inline(always)]
    pub const fn set_rom_rev_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for DeviceId0 {
    #[inline(always)]
    fn default() -> DeviceId0 {
        DeviceId0(0)
    }
}
impl core::fmt::Debug for DeviceId0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceId0")
            .field("rom_rev_minor", &self.rom_rev_minor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceId0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceId0 {{ rom_rev_minor: {=u8:?} }}",
            self.rom_rev_minor()
        )
    }
}
#[doc = "Device Type."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DeviceType(pub u32);
impl DeviceType {
    #[doc = "Indicates DEVICE TYPE."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates DEVICE TYPE."]
    #[inline(always)]
    pub const fn set_device_type(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DeviceType {
    #[inline(always)]
    fn default() -> DeviceType {
        DeviceType(0)
    }
}
impl core::fmt::Debug for DeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DeviceType")
            .field("device_type", &self.device_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DeviceType {{ device_type: {=u32:?} }}",
            self.device_type()
        )
    }
}
#[doc = "Chip Revision ID and Number."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dieid(pub u32);
impl Dieid {
    #[doc = "Chip minor revision."]
    #[must_use]
    #[inline(always)]
    pub const fn minor_revision(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip minor revision."]
    #[inline(always)]
    pub const fn set_minor_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Chip major revision."]
    #[must_use]
    #[inline(always)]
    pub const fn major_revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Chip major revision."]
    #[inline(always)]
    pub const fn set_major_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Chip number."]
    #[must_use]
    #[inline(always)]
    pub const fn mco_num_in_die_id(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Chip number."]
    #[inline(always)]
    pub const fn set_mco_num_in_die_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 8usize)) | (((val as u32) & 0x000f_ffff) << 8usize);
    }
}
impl Default for Dieid {
    #[inline(always)]
    fn default() -> Dieid {
        Dieid(0)
    }
}
impl core::fmt::Debug for Dieid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dieid")
            .field("minor_revision", &self.minor_revision())
            .field("major_revision", &self.major_revision())
            .field("mco_num_in_die_id", &self.mco_num_in_die_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dieid {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dieid {{ minor_revision: {=u8:?}, major_revision: {=u8:?}, mco_num_in_die_id: {=u32:?} }}",
            self.minor_revision(),
            self.major_revision(),
            self.mco_num_in_die_id()
        )
    }
}
#[doc = "RAM ECC Enable Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EccEnableCtrl(pub u32);
impl EccEnableCtrl {
    #[doc = "RAMA ECC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn rama_ecc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RAMA ECC enable."]
    #[inline(always)]
    pub const fn set_rama_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RAMB and RAMX ECC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramb_ramx_ecc_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RAMB and RAMX ECC enable."]
    #[inline(always)]
    pub const fn set_ramb_ramx_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "RAMD and RAMC ECC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramd_ramc_ecc_enable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "RAMD and RAMC ECC enable."]
    #[inline(always)]
    pub const fn set_ramd_ramc_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RAMF and RAME ECC enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ramf_rame_ecc_enable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RAMF and RAME ECC enable."]
    #[inline(always)]
    pub const fn set_ramf_rame_ecc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for EccEnableCtrl {
    #[inline(always)]
    fn default() -> EccEnableCtrl {
        EccEnableCtrl(0)
    }
}
impl core::fmt::Debug for EccEnableCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EccEnableCtrl")
            .field("rama_ecc_enable", &self.rama_ecc_enable())
            .field("ramb_ramx_ecc_enable", &self.ramb_ramx_ecc_enable())
            .field("ramd_ramc_ecc_enable", &self.ramd_ramc_ecc_enable())
            .field("ramf_rame_ecc_enable", &self.ramf_rame_ecc_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EccEnableCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EccEnableCtrl {{ rama_ecc_enable: {=bool:?}, ramb_ramx_ecc_enable: {=bool:?}, ramd_ramc_ecc_enable: {=bool:?}, ramf_rame_ecc_enable: {=bool:?} }}",
            self.rama_ecc_enable(),
            self.ramb_ramx_ecc_enable(),
            self.ramd_ramc_ecc_enable(),
            self.ramf_rame_ecc_enable()
        )
    }
}
#[doc = "Boot state captured during boot: Main ROM log."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog0(pub u32);
impl ElsAsBootLog0 {
    #[doc = "Boot image source used during this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn boot_image(&self) -> BootImage {
        let val = (self.0 >> 0usize) & 0x0f;
        BootImage::from_bits(val as u8)
    }
    #[doc = "Boot image source used during this boot."]
    #[inline(always)]
    pub const fn set_boot_image(&mut self, val: BootImage) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "CMAC verify is used instead of ECDSA verify on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn cmac(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "CMAC verify is used instead of ECDSA verify on this boot."]
    #[inline(always)]
    pub const fn set_cmac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ECDSA P-384 verification is done on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn ecdsa(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "ECDSA P-384 verification is done on this boot."]
    #[inline(always)]
    pub const fn set_ecdsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Off-chip Prince is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn off_chip(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Off-chip Prince is enabled during boot."]
    #[inline(always)]
    pub const fn set_off_chip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "On-chip Prince is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn on_chip(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "On-chip Prince is enabled during boot."]
    #[inline(always)]
    pub const fn set_on_chip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "CDI based device keys are derived for CSR harvesting on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn cdi_csr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "CDI based device keys are derived for CSR harvesting on this boot."]
    #[inline(always)]
    pub const fn set_cdi_csr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "CDI per DICE specification is computed on this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn cdi_dice(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "CDI per DICE specification is computed on this boot."]
    #[inline(always)]
    pub const fn set_cdi_dice(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "TrustZone preset data is loaded during this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn trustzone(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "TrustZone preset data is loaded during this boot."]
    #[inline(always)]
    pub const fn set_trustzone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Debug authentication done in this session prior to boot."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_auth(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Debug authentication done in this session prior to boot."]
    #[inline(always)]
    pub const fn set_debug_auth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "ITRC zeroize event is handled in this session of boot."]
    #[must_use]
    #[inline(always)]
    pub const fn itrc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "ITRC zeroize event is handled in this session of boot."]
    #[inline(always)]
    pub const fn set_itrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Digital glitch detector is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn dig_gdet(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Digital glitch detector is enabled during boot."]
    #[inline(always)]
    pub const fn set_dig_gdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Analog glitch detector is enabled during boot."]
    #[must_use]
    #[inline(always)]
    pub const fn ana_gdet(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Analog glitch detector is enabled during boot."]
    #[inline(always)]
    pub const fn set_ana_gdet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Boot from deep-power down state."]
    #[must_use]
    #[inline(always)]
    pub const fn deep_pd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Boot from deep-power down state."]
    #[inline(always)]
    pub const fn set_deep_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Last low-power mode value. ROM copies SPC_LP_MODE field from SPC->SC\\[7:4\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn low_power(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Last low-power mode value. ROM copies SPC_LP_MODE field from SPC->SC\\[7:4\\]."]
    #[inline(always)]
    pub const fn set_low_power(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "ISP pin state at boot time. ROM copies CMC->MR0\\[0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn isp(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ISP pin state at boot time. ROM copies CMC->MR0\\[0\\]."]
    #[inline(always)]
    pub const fn set_isp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ElsAsBootLog0 {
    #[inline(always)]
    fn default() -> ElsAsBootLog0 {
        ElsAsBootLog0(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog0")
            .field("boot_image", &self.boot_image())
            .field("cmac", &self.cmac())
            .field("ecdsa", &self.ecdsa())
            .field("off_chip", &self.off_chip())
            .field("on_chip", &self.on_chip())
            .field("cdi_csr", &self.cdi_csr())
            .field("cdi_dice", &self.cdi_dice())
            .field("trustzone", &self.trustzone())
            .field("debug_auth", &self.debug_auth())
            .field("itrc", &self.itrc())
            .field("dig_gdet", &self.dig_gdet())
            .field("ana_gdet", &self.ana_gdet())
            .field("deep_pd", &self.deep_pd())
            .field("low_power", &self.low_power())
            .field("isp", &self.isp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog0 {{ boot_image: {:?}, cmac: {=bool:?}, ecdsa: {=bool:?}, off_chip: {=bool:?}, on_chip: {=bool:?}, cdi_csr: {=bool:?}, cdi_dice: {=bool:?}, trustzone: {=bool:?}, debug_auth: {=bool:?}, itrc: {=bool:?}, dig_gdet: {=bool:?}, ana_gdet: {=bool:?}, deep_pd: {=bool:?}, low_power: {=u8:?}, isp: {=bool:?} }}",
            self.boot_image(),
            self.cmac(),
            self.ecdsa(),
            self.off_chip(),
            self.on_chip(),
            self.cdi_csr(),
            self.cdi_dice(),
            self.trustzone(),
            self.debug_auth(),
            self.itrc(),
            self.dig_gdet(),
            self.ana_gdet(),
            self.deep_pd(),
            self.low_power(),
            self.isp()
        )
    }
}
#[doc = "Boot state captured during boot: Library log."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog1(pub u32);
impl ElsAsBootLog1 {
    #[doc = "RoTK index used for this boot."]
    #[must_use]
    #[inline(always)]
    pub const fn ro_tk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "RoTK index used for this boot."]
    #[inline(always)]
    pub const fn set_ro_tk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "FIPS self-test is executed and PASS during this boot. When a bit is set, means self-test is executed and it FAILS. When a bit is clear, means corresponding self-test is executed and PASS or it is not executed."]
    #[must_use]
    #[inline(always)]
    pub const fn fips(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0xff;
        val as u8
    }
    #[doc = "FIPS self-test is executed and PASS during this boot. When a bit is set, means self-test is executed and it FAILS. When a bit is clear, means corresponding self-test is executed and PASS or it is not executed."]
    #[inline(always)]
    pub const fn set_fips(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 2usize)) | (((val as u32) & 0xff) << 2usize);
    }
    #[doc = "SB3 type (valid after nboot_sb3_load_manifest())."]
    #[must_use]
    #[inline(always)]
    pub const fn sb3(&self) -> Sb3 {
        let val = (self.0 >> 10usize) & 0x03;
        Sb3::from_bits(val as u8)
    }
    #[doc = "SB3 type (valid after nboot_sb3_load_manifest())."]
    #[inline(always)]
    pub const fn set_sb3(&mut self, val: Sb3) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
}
impl Default for ElsAsBootLog1 {
    #[inline(always)]
    fn default() -> ElsAsBootLog1 {
        ElsAsBootLog1(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog1")
            .field("ro_tk", &self.ro_tk())
            .field("fips", &self.fips())
            .field("sb3", &self.sb3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog1 {{ ro_tk: {=u8:?}, fips: {=u8:?}, sb3: {:?} }}",
            self.ro_tk(),
            self.fips(),
            self.sb3()
        )
    }
}
#[doc = "Boot state captured during boot: Hardware status signals log."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog2(pub u32);
impl ElsAsBootLog2 {
    #[doc = "CMC->SRS\\[5:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn cmc_srs0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "CMC->SRS\\[5:0\\]."]
    #[inline(always)]
    pub const fn set_cmc_srs0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "VBAT->STATUSA\\[1:0\\] | ~VBAT->STATUSB\\[1:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn vbat_status0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "VBAT->STATUSA\\[1:0\\] | ~VBAT->STATUSB\\[1:0\\]."]
    #[inline(always)]
    pub const fn set_vbat_status0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "CMC->SRS\\[16:8\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn cmc_srs1(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x01ff;
        val as u16
    }
    #[doc = "CMC->SRS\\[16:8\\]."]
    #[inline(always)]
    pub const fn set_cmc_srs1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 8usize)) | (((val as u32) & 0x01ff) << 8usize);
    }
    #[doc = "VBAT->STATUSA\\[11:6\\] | ~VBAT->STATUSB\\[11:6\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn vbat_status1(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "VBAT->STATUSA\\[11:6\\] | ~VBAT->STATUSB\\[11:6\\]."]
    #[inline(always)]
    pub const fn set_vbat_status1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "CMC->SRS\\[31:24\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn cmc_srs2(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "CMC->SRS\\[31:24\\]."]
    #[inline(always)]
    pub const fn set_cmc_srs2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ElsAsBootLog2 {
    #[inline(always)]
    fn default() -> ElsAsBootLog2 {
        ElsAsBootLog2(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog2")
            .field("cmc_srs0", &self.cmc_srs0())
            .field("vbat_status0", &self.vbat_status0())
            .field("cmc_srs1", &self.cmc_srs1())
            .field("vbat_status1", &self.vbat_status1())
            .field("cmc_srs2", &self.cmc_srs2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog2 {{ cmc_srs0: {=u8:?}, vbat_status0: {=u8:?}, cmc_srs1: {=u16:?}, vbat_status1: {=u8:?}, cmc_srs2: {=u8:?} }}",
            self.cmc_srs0(),
            self.vbat_status0(),
            self.cmc_srs1(),
            self.vbat_status1(),
            self.cmc_srs2()
        )
    }
}
#[doc = "Boot state captured during boot: Security log."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsBootLog3(pub u32);
impl ElsAsBootLog3 {
    #[doc = "CFPA->ERR_AUTH_FAIL_COUNT\\[7:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn err_auth_fail_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "CFPA->ERR_AUTH_FAIL_COUNT\\[7:0\\]."]
    #[inline(always)]
    pub const fn set_err_auth_fail_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "CFPA->ERR_ITRC_COUNT\\[7:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn err_itrc_count(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "CFPA->ERR_ITRC_COUNT\\[7:0\\]."]
    #[inline(always)]
    pub const fn set_err_itrc_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ElsAsBootLog3 {
    #[inline(always)]
    fn default() -> ElsAsBootLog3 {
        ElsAsBootLog3(0)
    }
}
impl core::fmt::Debug for ElsAsBootLog3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsBootLog3")
            .field("err_auth_fail_count", &self.err_auth_fail_count())
            .field("err_itrc_count", &self.err_itrc_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsBootLog3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsBootLog3 {{ err_auth_fail_count: {=u8:?}, err_itrc_count: {=u8:?} }}",
            self.err_auth_fail_count(),
            self.err_itrc_count()
        )
    }
}
#[doc = "ELS AS Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg0(pub u32);
impl ElsAsCfg0 {
    #[doc = "LC state configuration bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LC state configuration bit."]
    #[inline(always)]
    pub const fn set_cfg_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_core_reset_enabled(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_core_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD IRQ are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_core_irq_enabled(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE LVD analog detector are turned on, and CORE LVD IRQ are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_core_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When WatchDog Timer 0 is activated, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_wdt0_enabled(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When WatchDog Timer 0 is activated, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_wdt0_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When Code WatchDog Timer 0 is activated, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_cwdt0_enabled(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "When Code WatchDog Timer 0 is activated, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_cwdt0_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "When either GDET is enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_els_gdet_enabled(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "When either GDET is enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_els_gdet_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When SPC analog glitch detect reset is enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_ana_gdet_reset_enabled(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC analog glitch detect reset is enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_ana_gdet_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When SPC analog glitch detect IRQ is enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_ana_gdet_irq_enabled(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC analog glitch detect IRQ is enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_ana_gdet_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When tamper detector is enabled in TDET, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_tamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When tamper detector is enabled in TDET, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_tamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vsys_reset_enabled(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vsys_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vddio_reset_enabled(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vddio_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vsys_irq_enabled(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS LVD analog detector are turned on and VSYS LVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vsys_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lvd_vddio_irq_enabled(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO LVD analog detector are turned on and VDDIO LVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lvd_vddio_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "When WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_wdt1_enabled(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "When WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_wdt1_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "When Code WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_cwdt1_enabled(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "When Code WatchDog Timer 1 is activated, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_cwdt1_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "When temperature tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_temptamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When temperature tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_temptamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "When voltage tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_voltamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "When voltage tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_voltamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "When light tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_lhttamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "When light tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_lhttamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When clk tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_clktamper_det_enabled(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "When clk tamper detector is enabled in VBAT, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_clktamper_det_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "When QK PUF \"qk_disable_enroll\" input is driven 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_qk_disable_enroll(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When QK PUF \"qk_disable_enroll\" input is driven 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_qk_disable_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "When QK PUF \"qk_disable_wrap\" input is driven 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_qk_disable_wrap(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "When QK PUF \"qk_disable_wrap\" input is driven 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_qk_disable_wrap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for ElsAsCfg0 {
    #[inline(always)]
    fn default() -> ElsAsCfg0 {
        ElsAsCfg0(0)
    }
}
impl core::fmt::Debug for ElsAsCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg0")
            .field("cfg_lc_state", &self.cfg_lc_state())
            .field(
                "cfg_lvd_core_reset_enabled",
                &self.cfg_lvd_core_reset_enabled(),
            )
            .field("cfg_lvd_core_irq_enabled", &self.cfg_lvd_core_irq_enabled())
            .field("cfg_wdt0_enabled", &self.cfg_wdt0_enabled())
            .field("cfg_cwdt0_enabled", &self.cfg_cwdt0_enabled())
            .field("cfg_els_gdet_enabled", &self.cfg_els_gdet_enabled())
            .field(
                "cfg_ana_gdet_reset_enabled",
                &self.cfg_ana_gdet_reset_enabled(),
            )
            .field("cfg_ana_gdet_irq_enabled", &self.cfg_ana_gdet_irq_enabled())
            .field("cfg_tamper_det_enabled", &self.cfg_tamper_det_enabled())
            .field(
                "cfg_lvd_vsys_reset_enabled",
                &self.cfg_lvd_vsys_reset_enabled(),
            )
            .field(
                "cfg_lvd_vddio_reset_enabled",
                &self.cfg_lvd_vddio_reset_enabled(),
            )
            .field("cfg_lvd_vsys_irq_enabled", &self.cfg_lvd_vsys_irq_enabled())
            .field(
                "cfg_lvd_vddio_irq_enabled",
                &self.cfg_lvd_vddio_irq_enabled(),
            )
            .field("cfg_wdt1_enabled", &self.cfg_wdt1_enabled())
            .field("cfg_cwdt1_enabled", &self.cfg_cwdt1_enabled())
            .field(
                "cfg_temptamper_det_enabled",
                &self.cfg_temptamper_det_enabled(),
            )
            .field(
                "cfg_voltamper_det_enabled",
                &self.cfg_voltamper_det_enabled(),
            )
            .field(
                "cfg_lhttamper_det_enabled",
                &self.cfg_lhttamper_det_enabled(),
            )
            .field(
                "cfg_clktamper_det_enabled",
                &self.cfg_clktamper_det_enabled(),
            )
            .field("cfg_qk_disable_enroll", &self.cfg_qk_disable_enroll())
            .field("cfg_qk_disable_wrap", &self.cfg_qk_disable_wrap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg0 {{ cfg_lc_state: {=u8:?}, cfg_lvd_core_reset_enabled: {=bool:?}, cfg_lvd_core_irq_enabled: {=bool:?}, cfg_wdt0_enabled: {=bool:?}, cfg_cwdt0_enabled: {=bool:?}, cfg_els_gdet_enabled: {=bool:?}, cfg_ana_gdet_reset_enabled: {=bool:?}, cfg_ana_gdet_irq_enabled: {=bool:?}, cfg_tamper_det_enabled: {=bool:?}, cfg_lvd_vsys_reset_enabled: {=bool:?}, cfg_lvd_vddio_reset_enabled: {=bool:?}, cfg_lvd_vsys_irq_enabled: {=bool:?}, cfg_lvd_vddio_irq_enabled: {=bool:?}, cfg_wdt1_enabled: {=bool:?}, cfg_cwdt1_enabled: {=bool:?}, cfg_temptamper_det_enabled: {=bool:?}, cfg_voltamper_det_enabled: {=bool:?}, cfg_lhttamper_det_enabled: {=bool:?}, cfg_clktamper_det_enabled: {=bool:?}, cfg_qk_disable_enroll: {=bool:?}, cfg_qk_disable_wrap: {=bool:?} }}",
            self.cfg_lc_state(),
            self.cfg_lvd_core_reset_enabled(),
            self.cfg_lvd_core_irq_enabled(),
            self.cfg_wdt0_enabled(),
            self.cfg_cwdt0_enabled(),
            self.cfg_els_gdet_enabled(),
            self.cfg_ana_gdet_reset_enabled(),
            self.cfg_ana_gdet_irq_enabled(),
            self.cfg_tamper_det_enabled(),
            self.cfg_lvd_vsys_reset_enabled(),
            self.cfg_lvd_vddio_reset_enabled(),
            self.cfg_lvd_vsys_irq_enabled(),
            self.cfg_lvd_vddio_irq_enabled(),
            self.cfg_wdt1_enabled(),
            self.cfg_cwdt1_enabled(),
            self.cfg_temptamper_det_enabled(),
            self.cfg_voltamper_det_enabled(),
            self.cfg_lhttamper_det_enabled(),
            self.cfg_clktamper_det_enabled(),
            self.cfg_qk_disable_enroll(),
            self.cfg_qk_disable_wrap()
        )
    }
}
#[doc = "ELS AS Configuration1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg1(pub u32);
impl ElsAsCfg1 {
    #[doc = "When CFG_SEC_ENA_SEC_CHK indicates state 0 or when DISABLE_STRICT_MODE bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_dis_strict_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When CFG_SEC_ENA_SEC_CHK indicates state 0 or when DISABLE_STRICT_MODE bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_dis_strict_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When the DISABLE_VIOLATION_ABORT bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_dis_viol_abort(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When the DISABLE_VIOLATION_ABORT bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_dis_viol_abort(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When the ENABLE_NS_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_ena_ns_priv_chk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When the ENABLE_NS_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_ena_ns_priv_chk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When the ENABLE_S_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_ena_s_priv_chk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When the ENABLE_S_PRIV_CHECK bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_ena_s_priv_chk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When the ENABLE_SECURE_CHECKING bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_ena_sec_chk(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When the ENABLE_SECURE_CHECKING bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_ena_sec_chk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When the IDAU_ALL_NS bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_idau_allns(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When the IDAU_ALL_NS bits in MISC_CTRL_REG and MISC_CTRL_DP_REG on the AHB secure controller are equal to 01, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_idau_allns(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When the LOCK_NS_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_ns_mpu(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_NS_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_ns_mpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When the LOCK_NS_VTOR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_ns_vtor(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_NS_VTOR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_ns_vtor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When the LOCK_S_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_s_mpu(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_S_MPU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_s_mpu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When the LOCK_S_VTAIRCR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_s_vtaircr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_S_VTAIRCR bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_s_vtaircr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When the LOCK_SAU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_sec_lock_sau(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When the LOCK_SAU bits in CPU0_LOCK_REG on the AHB secure controller are not equal to 10, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_sec_lock_sau(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "metal version."]
    #[must_use]
    #[inline(always)]
    pub const fn metal_version(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0xff;
        val as u8
    }
    #[doc = "metal version."]
    #[inline(always)]
    pub const fn set_metal_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 13usize)) | (((val as u32) & 0xff) << 13usize);
    }
    #[doc = "ROM patch version."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_patch_version(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "ROM patch version."]
    #[inline(always)]
    pub const fn set_rom_patch_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_core_reset_enabled(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_core_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD IRQ are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_core_irq_enabled(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC CORE HVD analog detector are turned on, and CORE HVD IRQ are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_core_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vsys_reset_enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vsys_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD reset are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vddio_reset_enabled(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD reset are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vddio_reset_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vsys_irq_enabled(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VSYS HVD analog detector are turned on and VSYS HVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vsys_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD irq are enabled, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_hvd_vddio_irq_enabled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When SPC VDDIO HVD analog detector are turned on and VDDIO HVD irq are enabled, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_cfg_hvd_vddio_irq_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ElsAsCfg1 {
    #[inline(always)]
    fn default() -> ElsAsCfg1 {
        ElsAsCfg1(0)
    }
}
impl core::fmt::Debug for ElsAsCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg1")
            .field("cfg_sec_dis_strict_mode", &self.cfg_sec_dis_strict_mode())
            .field("cfg_sec_dis_viol_abort", &self.cfg_sec_dis_viol_abort())
            .field("cfg_sec_ena_ns_priv_chk", &self.cfg_sec_ena_ns_priv_chk())
            .field("cfg_sec_ena_s_priv_chk", &self.cfg_sec_ena_s_priv_chk())
            .field("cfg_sec_ena_sec_chk", &self.cfg_sec_ena_sec_chk())
            .field("cfg_sec_idau_allns", &self.cfg_sec_idau_allns())
            .field("cfg_sec_lock_ns_mpu", &self.cfg_sec_lock_ns_mpu())
            .field("cfg_sec_lock_ns_vtor", &self.cfg_sec_lock_ns_vtor())
            .field("cfg_sec_lock_s_mpu", &self.cfg_sec_lock_s_mpu())
            .field("cfg_sec_lock_s_vtaircr", &self.cfg_sec_lock_s_vtaircr())
            .field("cfg_sec_lock_sau", &self.cfg_sec_lock_sau())
            .field("metal_version", &self.metal_version())
            .field("rom_patch_version", &self.rom_patch_version())
            .field(
                "cfg_hvd_core_reset_enabled",
                &self.cfg_hvd_core_reset_enabled(),
            )
            .field("cfg_hvd_core_irq_enabled", &self.cfg_hvd_core_irq_enabled())
            .field(
                "cfg_hvd_vsys_reset_enabled",
                &self.cfg_hvd_vsys_reset_enabled(),
            )
            .field(
                "cfg_hvd_vddio_reset_enabled",
                &self.cfg_hvd_vddio_reset_enabled(),
            )
            .field("cfg_hvd_vsys_irq_enabled", &self.cfg_hvd_vsys_irq_enabled())
            .field(
                "cfg_hvd_vddio_irq_enabled",
                &self.cfg_hvd_vddio_irq_enabled(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg1 {{ cfg_sec_dis_strict_mode: {=bool:?}, cfg_sec_dis_viol_abort: {=bool:?}, cfg_sec_ena_ns_priv_chk: {=bool:?}, cfg_sec_ena_s_priv_chk: {=bool:?}, cfg_sec_ena_sec_chk: {=bool:?}, cfg_sec_idau_allns: {=bool:?}, cfg_sec_lock_ns_mpu: {=bool:?}, cfg_sec_lock_ns_vtor: {=bool:?}, cfg_sec_lock_s_mpu: {=bool:?}, cfg_sec_lock_s_vtaircr: {=bool:?}, cfg_sec_lock_sau: {=bool:?}, metal_version: {=u8:?}, rom_patch_version: {=u8:?}, cfg_hvd_core_reset_enabled: {=bool:?}, cfg_hvd_core_irq_enabled: {=bool:?}, cfg_hvd_vsys_reset_enabled: {=bool:?}, cfg_hvd_vddio_reset_enabled: {=bool:?}, cfg_hvd_vsys_irq_enabled: {=bool:?}, cfg_hvd_vddio_irq_enabled: {=bool:?} }}",
            self.cfg_sec_dis_strict_mode(),
            self.cfg_sec_dis_viol_abort(),
            self.cfg_sec_ena_ns_priv_chk(),
            self.cfg_sec_ena_s_priv_chk(),
            self.cfg_sec_ena_sec_chk(),
            self.cfg_sec_idau_allns(),
            self.cfg_sec_lock_ns_mpu(),
            self.cfg_sec_lock_ns_vtor(),
            self.cfg_sec_lock_s_mpu(),
            self.cfg_sec_lock_s_vtaircr(),
            self.cfg_sec_lock_sau(),
            self.metal_version(),
            self.rom_patch_version(),
            self.cfg_hvd_core_reset_enabled(),
            self.cfg_hvd_core_irq_enabled(),
            self.cfg_hvd_vsys_reset_enabled(),
            self.cfg_hvd_vddio_reset_enabled(),
            self.cfg_hvd_vsys_irq_enabled(),
            self.cfg_hvd_vddio_irq_enabled()
        )
    }
}
#[doc = "ELS AS Configuration2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg2(pub u32);
impl ElsAsCfg2 {
    #[doc = "ELS configuration command enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_els_cmd_en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ELS configuration command enable bit."]
    #[inline(always)]
    pub const fn set_cfg_els_cmd_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsAsCfg2 {
    #[inline(always)]
    fn default() -> ElsAsCfg2 {
        ElsAsCfg2(0)
    }
}
impl core::fmt::Debug for ElsAsCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg2")
            .field("cfg_els_cmd_en", &self.cfg_els_cmd_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg2 {{ cfg_els_cmd_en: {=u32:?} }}",
            self.cfg_els_cmd_en()
        )
    }
}
#[doc = "ELS AS Configuration3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsCfg3(pub u32);
impl ElsAsCfg3 {
    #[doc = "Device type identification data."]
    #[must_use]
    #[inline(always)]
    pub const fn device_type(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Device type identification data."]
    #[inline(always)]
    pub const fn set_device_type(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsAsCfg3 {
    #[inline(always)]
    fn default() -> ElsAsCfg3 {
        ElsAsCfg3(0)
    }
}
impl core::fmt::Debug for ElsAsCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsCfg3")
            .field("device_type", &self.device_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsCfg3 {{ device_type: {=u32:?} }}",
            self.device_type()
        )
    }
}
#[doc = "ELS AS Flag0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsFlag0(pub u32);
impl ElsAsFlag0 {
    #[doc = "This flag bit is set as 1 when DAP enables AP0 for CPU0 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ap_enable_cpu0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP0 for CPU0 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ap_enable_cpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP1 for CPU1 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ap_enable_cpu1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP1 for CPU1 (CM33) debug access. The register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ap_enable_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP3 for DSP (CoolFlux) debug access. The register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ap_enable_dsp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when DAP enables AP3 for DSP (CoolFlux) debug access. The register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ap_enable_dsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "OTPC can output attack_detect signal when it detects attack when load shadow registers. The output will be cleared by reset. ELS_AS_FLAG is reset by PoR, so the status can be recorded."]
    #[must_use]
    #[inline(always)]
    pub const fn efuse_attack_detect(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "OTPC can output attack_detect signal when it detects attack when load shadow registers. The output will be cleared by reset. ELS_AS_FLAG is reset by PoR, so the status can be recorded."]
    #[inline(always)]
    pub const fn set_efuse_attack_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "This flag register is set 1 when VDD_CORE LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lvd_core_occured(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "This flag register is set 1 when VDD_CORE LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lvd_core_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt0_reset_occured(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_wdt0_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt0_reset_occured(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_cwdt0_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt0_irq_occured(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_wdt0_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt0_irq_occured(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 0 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cwdt0_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This flag bit is set as 1 when QK_ERROR is flagged from QK PUF block. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_qk_error(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when QK_ERROR is flagged from QK PUF block. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_qk_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This flag bit is set as 1 when GDET error is flagged. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_els_glitch_detected(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when GDET error is flagged. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_els_glitch_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This flag bit is set as 1 when ANALOG GDET error is flagged in SYSCON block. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_ana_glitch_detected(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when ANALOG GDET error is flagged in SYSCON block. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_ana_glitch_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "This flag bit is set as 1 when tamper event is flagged from TDET. This register is cleared 0 by AO domain POR or by PMC reset event, if tamper detection event is cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_tamper_event_detected(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when tamper event is flagged from TDET. This register is cleared 0 by AO domain POR or by PMC reset event, if tamper detection event is cleared by software."]
    #[inline(always)]
    pub const fn set_flag_tamper_event_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This flag bit is set as 1 when FLASH controller indicates ECC error. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_flash_ecc_invalid(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when FLASH controller indicates ECC error. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_flash_ecc_invalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "This flag bit is set as 1 when security violation is indicated from FLASH sub-system or AHB bus matrix."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_sec_viol_irq_ocurred(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when security violation is indicated from FLASH sub-system or AHB bus matrix."]
    #[inline(always)]
    pub const fn set_flag_sec_viol_irq_ocurred(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure code transactions. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cpu0_ns_c_acc_occured(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure code transactions. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cpu0_ns_c_acc_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure data transactions. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cpu0_ns_d_acc_occured(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when CPU0 (CM33) makes non-secure data transactions. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cpu0_ns_d_acc_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "This flag register is set 1 when VDD_SYS LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lvd_vsys_occured(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "This flag register is set 1 when VDD_SYS LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lvd_vsys_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This flag register is set 1 when VDD LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lvd_vddio_occured(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "This flag register is set 1 when VDD LVD event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lvd_vddio_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt1_reset_occured(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_wdt1_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt1_reset_occured(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 reset is enabled and reset event is triggered. This register is cleared 0 by AO domain POR."]
    #[inline(always)]
    pub const fn set_flag_cwdt1_reset_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_wdt1_irq_occured(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_wdt1_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_cwdt1_irq_occured(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when Code WatchDog Timer 1 IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_cwdt1_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This flag bit is set as 1 when temperature temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_temptamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when temperature temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_temptamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This flag bit is set as 1 when voltage temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_voltamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when voltage temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_voltamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "This flag bit is set as 1 when light temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_lhttamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when light temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_lhttamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This flag bit is set as 1 when clock temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_clktamper_det_irq_occured(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when clock temper IRQ is enabled and IRQ event is triggered. This register is cleared 0 by PMC reset event."]
    #[inline(always)]
    pub const fn set_flag_clktamper_det_irq_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for ElsAsFlag0 {
    #[inline(always)]
    fn default() -> ElsAsFlag0 {
        ElsAsFlag0(0)
    }
}
impl core::fmt::Debug for ElsAsFlag0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsFlag0")
            .field("flag_ap_enable_cpu0", &self.flag_ap_enable_cpu0())
            .field("flag_ap_enable_cpu1", &self.flag_ap_enable_cpu1())
            .field("flag_ap_enable_dsp", &self.flag_ap_enable_dsp())
            .field("efuse_attack_detect", &self.efuse_attack_detect())
            .field("flag_lvd_core_occured", &self.flag_lvd_core_occured())
            .field("flag_wdt0_reset_occured", &self.flag_wdt0_reset_occured())
            .field("flag_cwdt0_reset_occured", &self.flag_cwdt0_reset_occured())
            .field("flag_wdt0_irq_occured", &self.flag_wdt0_irq_occured())
            .field("flag_cwdt0_irq_occured", &self.flag_cwdt0_irq_occured())
            .field("flag_qk_error", &self.flag_qk_error())
            .field("flag_els_glitch_detected", &self.flag_els_glitch_detected())
            .field("flag_ana_glitch_detected", &self.flag_ana_glitch_detected())
            .field(
                "flag_tamper_event_detected",
                &self.flag_tamper_event_detected(),
            )
            .field("flag_flash_ecc_invalid", &self.flag_flash_ecc_invalid())
            .field(
                "flag_sec_viol_irq_ocurred",
                &self.flag_sec_viol_irq_ocurred(),
            )
            .field(
                "flag_cpu0_ns_c_acc_occured",
                &self.flag_cpu0_ns_c_acc_occured(),
            )
            .field(
                "flag_cpu0_ns_d_acc_occured",
                &self.flag_cpu0_ns_d_acc_occured(),
            )
            .field("flag_lvd_vsys_occured", &self.flag_lvd_vsys_occured())
            .field("flag_lvd_vddio_occured", &self.flag_lvd_vddio_occured())
            .field("flag_wdt1_reset_occured", &self.flag_wdt1_reset_occured())
            .field("flag_cwdt1_reset_occured", &self.flag_cwdt1_reset_occured())
            .field("flag_wdt1_irq_occured", &self.flag_wdt1_irq_occured())
            .field("flag_cwdt1_irq_occured", &self.flag_cwdt1_irq_occured())
            .field(
                "flag_temptamper_det_irq_occured",
                &self.flag_temptamper_det_irq_occured(),
            )
            .field(
                "flag_voltamper_det_irq_occured",
                &self.flag_voltamper_det_irq_occured(),
            )
            .field(
                "flag_lhttamper_det_irq_occured",
                &self.flag_lhttamper_det_irq_occured(),
            )
            .field(
                "flag_clktamper_det_irq_occured",
                &self.flag_clktamper_det_irq_occured(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsFlag0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsFlag0 {{ flag_ap_enable_cpu0: {=bool:?}, flag_ap_enable_cpu1: {=bool:?}, flag_ap_enable_dsp: {=bool:?}, efuse_attack_detect: {=bool:?}, flag_lvd_core_occured: {=bool:?}, flag_wdt0_reset_occured: {=bool:?}, flag_cwdt0_reset_occured: {=bool:?}, flag_wdt0_irq_occured: {=bool:?}, flag_cwdt0_irq_occured: {=bool:?}, flag_qk_error: {=bool:?}, flag_els_glitch_detected: {=bool:?}, flag_ana_glitch_detected: {=bool:?}, flag_tamper_event_detected: {=bool:?}, flag_flash_ecc_invalid: {=bool:?}, flag_sec_viol_irq_ocurred: {=bool:?}, flag_cpu0_ns_c_acc_occured: {=bool:?}, flag_cpu0_ns_d_acc_occured: {=bool:?}, flag_lvd_vsys_occured: {=bool:?}, flag_lvd_vddio_occured: {=bool:?}, flag_wdt1_reset_occured: {=bool:?}, flag_cwdt1_reset_occured: {=bool:?}, flag_wdt1_irq_occured: {=bool:?}, flag_cwdt1_irq_occured: {=bool:?}, flag_temptamper_det_irq_occured: {=bool:?}, flag_voltamper_det_irq_occured: {=bool:?}, flag_lhttamper_det_irq_occured: {=bool:?}, flag_clktamper_det_irq_occured: {=bool:?} }}",
            self.flag_ap_enable_cpu0(),
            self.flag_ap_enable_cpu1(),
            self.flag_ap_enable_dsp(),
            self.efuse_attack_detect(),
            self.flag_lvd_core_occured(),
            self.flag_wdt0_reset_occured(),
            self.flag_cwdt0_reset_occured(),
            self.flag_wdt0_irq_occured(),
            self.flag_cwdt0_irq_occured(),
            self.flag_qk_error(),
            self.flag_els_glitch_detected(),
            self.flag_ana_glitch_detected(),
            self.flag_tamper_event_detected(),
            self.flag_flash_ecc_invalid(),
            self.flag_sec_viol_irq_ocurred(),
            self.flag_cpu0_ns_c_acc_occured(),
            self.flag_cpu0_ns_d_acc_occured(),
            self.flag_lvd_vsys_occured(),
            self.flag_lvd_vddio_occured(),
            self.flag_wdt1_reset_occured(),
            self.flag_cwdt1_reset_occured(),
            self.flag_wdt1_irq_occured(),
            self.flag_cwdt1_irq_occured(),
            self.flag_temptamper_det_irq_occured(),
            self.flag_voltamper_det_irq_occured(),
            self.flag_lhttamper_det_irq_occured(),
            self.flag_clktamper_det_irq_occured()
        )
    }
}
#[doc = "ELS AS Flag1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsFlag1(pub u32);
impl ElsAsFlag1 {
    #[doc = "This flag bit is set as 1 when HVD from VDD_CORE power domain is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_hvd_core_occured(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_CORE power domain is triggered."]
    #[inline(always)]
    pub const fn set_flag_hvd_core_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_SYS power domain is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_hvd_vsys_occured(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD_SYS power domain is triggered."]
    #[inline(always)]
    pub const fn set_flag_hvd_vsys_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD power domain is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn flag_hvd_vddio_occured(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "This flag bit is set as 1 when HVD from VDD power domain is triggered."]
    #[inline(always)]
    pub const fn set_flag_hvd_vddio_occured(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ElsAsFlag1 {
    #[inline(always)]
    fn default() -> ElsAsFlag1 {
        ElsAsFlag1(0)
    }
}
impl core::fmt::Debug for ElsAsFlag1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsFlag1")
            .field("flag_hvd_core_occured", &self.flag_hvd_core_occured())
            .field("flag_hvd_vsys_occured", &self.flag_hvd_vsys_occured())
            .field("flag_hvd_vddio_occured", &self.flag_hvd_vddio_occured())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsFlag1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsFlag1 {{ flag_hvd_core_occured: {=bool:?}, flag_hvd_vsys_occured: {=bool:?}, flag_hvd_vddio_occured: {=bool:?} }}",
            self.flag_hvd_core_occured(),
            self.flag_hvd_vsys_occured(),
            self.flag_hvd_vddio_occured()
        )
    }
}
#[doc = "ELS AS State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsSt0(pub u32);
impl ElsAsSt0 {
    #[doc = "TEMPORAL_STATE\\[3:0\\] in the ELS_TEMPORAL_STATE register reflects this register."]
    #[must_use]
    #[inline(always)]
    pub const fn st_temporal_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "TEMPORAL_STATE\\[3:0\\] in the ELS_TEMPORAL_STATE register reflects this register."]
    #[inline(always)]
    pub const fn set_st_temporal_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "When CPU0 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_dbgen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu0_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When CPU0 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_niden(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu0_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When CPU0 (CM33) \"spiden\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_spiden(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"spiden\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu0_spiden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When CPU0 (CM33) \"spniden\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu0_spniden(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU0 (CM33) \"spniden\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu0_spniden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "When CPU1 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu1_dbgen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU1 (CM33) \"deben\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu1_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "When CPU1 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_cpu1_niden(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When CPU1 (CM33) \"niden\" input is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_cpu1_niden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "When DAP to AP0 for CPU0 (CM33) debug access is allowed, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dap_enable_cpu0(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When DAP to AP0 for CPU0 (CM33) debug access is allowed, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_dap_enable_cpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When DAP to AP1 for CPU1 (CM33) debug access is allowed, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dap_enable_cpu1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When DAP to AP1 for CPU1 (CM33) debug access is allowed, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_dap_enable_cpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "When DAP to AP3 for DSP (CoolFlux) debug access is allowed, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dap_enable_dsp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "When DAP to AP3 for DSP (CoolFlux) debug access is allowed, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_dap_enable_dsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When JTAG TAP access is allowed, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_allow_test_access(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "When JTAG TAP access is allowed, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_allow_test_access(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "When XO32K oscillation fail flag is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_xo32k_failed(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "When XO32K oscillation fail flag is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_xo32k_failed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "When XO40M oscillation fail flag is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_xo40m_failed(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When XO40M oscillation fail flag is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_xo40m_failed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When IFR load fail flag is state 1, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_ifr_load_failed(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When IFR load fail flag is state 1, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_ifr_load_failed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "GLITCH_DETECT_FLAG is state of 4-bit Glitch Ripple Counter output."]
    #[must_use]
    #[inline(always)]
    pub const fn st_glitch_detect_flag(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "GLITCH_DETECT_FLAG is state of 4-bit Glitch Ripple Counter output."]
    #[inline(always)]
    pub const fn set_st_glitch_detect_flag(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
}
impl Default for ElsAsSt0 {
    #[inline(always)]
    fn default() -> ElsAsSt0 {
        ElsAsSt0(0)
    }
}
impl core::fmt::Debug for ElsAsSt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsSt0")
            .field("st_temporal_state", &self.st_temporal_state())
            .field("st_cpu0_dbgen", &self.st_cpu0_dbgen())
            .field("st_cpu0_niden", &self.st_cpu0_niden())
            .field("st_cpu0_spiden", &self.st_cpu0_spiden())
            .field("st_cpu0_spniden", &self.st_cpu0_spniden())
            .field("st_cpu1_dbgen", &self.st_cpu1_dbgen())
            .field("st_cpu1_niden", &self.st_cpu1_niden())
            .field("st_dap_enable_cpu0", &self.st_dap_enable_cpu0())
            .field("st_dap_enable_cpu1", &self.st_dap_enable_cpu1())
            .field("st_dap_enable_dsp", &self.st_dap_enable_dsp())
            .field("st_allow_test_access", &self.st_allow_test_access())
            .field("st_xo32k_failed", &self.st_xo32k_failed())
            .field("st_xo40m_failed", &self.st_xo40m_failed())
            .field("st_ifr_load_failed", &self.st_ifr_load_failed())
            .field("st_glitch_detect_flag", &self.st_glitch_detect_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsSt0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsSt0 {{ st_temporal_state: {=u8:?}, st_cpu0_dbgen: {=bool:?}, st_cpu0_niden: {=bool:?}, st_cpu0_spiden: {=bool:?}, st_cpu0_spniden: {=bool:?}, st_cpu1_dbgen: {=bool:?}, st_cpu1_niden: {=bool:?}, st_dap_enable_cpu0: {=bool:?}, st_dap_enable_cpu1: {=bool:?}, st_dap_enable_dsp: {=bool:?}, st_allow_test_access: {=bool:?}, st_xo32k_failed: {=bool:?}, st_xo40m_failed: {=bool:?}, st_ifr_load_failed: {=bool:?}, st_glitch_detect_flag: {=u8:?} }}",
            self.st_temporal_state(),
            self.st_cpu0_dbgen(),
            self.st_cpu0_niden(),
            self.st_cpu0_spiden(),
            self.st_cpu0_spniden(),
            self.st_cpu1_dbgen(),
            self.st_cpu1_niden(),
            self.st_dap_enable_cpu0(),
            self.st_dap_enable_cpu1(),
            self.st_dap_enable_dsp(),
            self.st_allow_test_access(),
            self.st_xo32k_failed(),
            self.st_xo40m_failed(),
            self.st_ifr_load_failed(),
            self.st_glitch_detect_flag()
        )
    }
}
#[doc = "ELS AS State1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAsSt1(pub u32);
impl ElsAsSt1 {
    #[doc = "These register bits indicate the state of \"qk_puf_score\\[3:0\\]\" outputs from QK PUF block."]
    #[must_use]
    #[inline(always)]
    pub const fn st_qk_puf_score(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These register bits indicate the state of \"qk_puf_score\\[3:0\\]\" outputs from QK PUF block."]
    #[inline(always)]
    pub const fn set_st_qk_puf_score(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This register bit indicates the state of \"qk_zeroized\" output from QK PUF block."]
    #[must_use]
    #[inline(always)]
    pub const fn st_qk_zeroized(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This register bit indicates the state of \"qk_zeroized\" output from QK PUF block."]
    #[inline(always)]
    pub const fn set_st_qk_zeroized(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "When MAIN_CLK is running from external clock source either XO32M, XO32K or GPIO CLKIN, this bit indicates state 1."]
    #[must_use]
    #[inline(always)]
    pub const fn st_main_clk_is_ext(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "When MAIN_CLK is running from external clock source either XO32M, XO32K or GPIO CLKIN, this bit indicates state 1."]
    #[inline(always)]
    pub const fn set_st_main_clk_is_ext(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "VOUT\\[1:0\\] setting on DCDC0 register in SPC block will reflect to this register. Default is 1.0V."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dcdc_vout(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "VOUT\\[1:0\\] setting on DCDC0 register in SPC block will reflect to this register. Default is 1.0V."]
    #[inline(always)]
    pub const fn set_st_dcdc_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "DCDC drive strength setting. Default is normal drive."]
    #[must_use]
    #[inline(always)]
    pub const fn st_dcdc_ds(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "DCDC drive strength setting. Default is normal drive."]
    #[inline(always)]
    pub const fn set_st_dcdc_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "ISP pin status during boot. By default ISP pin is pulled up. If want to enter ISP mode during boot, ISP pin should be pull down when out of reset."]
    #[must_use]
    #[inline(always)]
    pub const fn st_boot_mode(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "ISP pin status during boot. By default ISP pin is pulled up. If want to enter ISP mode during boot, ISP pin should be pull down when out of reset."]
    #[inline(always)]
    pub const fn set_st_boot_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "BOOT_RETRY_CNT\\[3:0\\] in the ELS_BOOT_RETRY_CNT register reflects this register."]
    #[must_use]
    #[inline(always)]
    pub const fn st_boot_retry_cnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "BOOT_RETRY_CNT\\[3:0\\] in the ELS_BOOT_RETRY_CNT register reflects this register."]
    #[inline(always)]
    pub const fn set_st_boot_retry_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "VOUT\\[1:0\\] setting on LDO Core register in SPC block will reflect to this register. Default is 1.0V."]
    #[must_use]
    #[inline(always)]
    pub const fn st_ldo_core_vout(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "VOUT\\[1:0\\] setting on LDO Core register in SPC block will reflect to this register. Default is 1.0V."]
    #[inline(always)]
    pub const fn set_st_ldo_core_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "LDO_CORE drive strength setting. Default is normal drive."]
    #[must_use]
    #[inline(always)]
    pub const fn st_ldo_core_ds(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "LDO_CORE drive strength setting. Default is normal drive."]
    #[inline(always)]
    pub const fn set_st_ldo_core_ds(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for ElsAsSt1 {
    #[inline(always)]
    fn default() -> ElsAsSt1 {
        ElsAsSt1(0)
    }
}
impl core::fmt::Debug for ElsAsSt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAsSt1")
            .field("st_qk_puf_score", &self.st_qk_puf_score())
            .field("st_qk_zeroized", &self.st_qk_zeroized())
            .field("st_main_clk_is_ext", &self.st_main_clk_is_ext())
            .field("st_dcdc_vout", &self.st_dcdc_vout())
            .field("st_dcdc_ds", &self.st_dcdc_ds())
            .field("st_boot_mode", &self.st_boot_mode())
            .field("st_boot_retry_cnt", &self.st_boot_retry_cnt())
            .field("st_ldo_core_vout", &self.st_ldo_core_vout())
            .field("st_ldo_core_ds", &self.st_ldo_core_ds())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAsSt1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAsSt1 {{ st_qk_puf_score: {=u8:?}, st_qk_zeroized: {=bool:?}, st_main_clk_is_ext: {=bool:?}, st_dcdc_vout: {=u8:?}, st_dcdc_ds: {=u8:?}, st_boot_mode: {=u8:?}, st_boot_retry_cnt: {=u8:?}, st_ldo_core_vout: {=u8:?}, st_ldo_core_ds: {=u8:?} }}",
            self.st_qk_puf_score(),
            self.st_qk_zeroized(),
            self.st_main_clk_is_ext(),
            self.st_dcdc_vout(),
            self.st_dcdc_ds(),
            self.st_boot_mode(),
            self.st_boot_retry_cnt(),
            self.st_ldo_core_vout(),
            self.st_ldo_core_ds()
        )
    }
}
#[doc = "ELS Asset Protection Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsAssetProt(pub u32);
impl ElsAssetProt {
    #[doc = "ELS asset protection. This field controls the asset protection port to the ELS module. Refer to the ELS chapter in the SRM for more details."]
    #[must_use]
    #[inline(always)]
    pub const fn asset_protection(&self) -> AssetProtection {
        let val = (self.0 >> 0usize) & 0x03;
        AssetProtection::from_bits(val as u8)
    }
    #[doc = "ELS asset protection. This field controls the asset protection port to the ELS module. Refer to the ELS chapter in the SRM for more details."]
    #[inline(always)]
    pub const fn set_asset_protection(&mut self, val: AssetProtection) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for ElsAssetProt {
    #[inline(always)]
    fn default() -> ElsAssetProt {
        ElsAssetProt(0)
    }
}
impl core::fmt::Debug for ElsAssetProt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsAssetProt")
            .field("asset_protection", &self.asset_protection())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsAssetProt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsAssetProt {{ asset_protection: {:?} }}",
            self.asset_protection()
        )
    }
}
#[doc = "Key Derivation Function Mask."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsKdfMask(pub u32);
impl ElsKdfMask {
    #[doc = "Key derivation function mask."]
    #[must_use]
    #[inline(always)]
    pub const fn kdf_mask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key derivation function mask."]
    #[inline(always)]
    pub const fn set_kdf_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for ElsKdfMask {
    #[inline(always)]
    fn default() -> ElsKdfMask {
        ElsKdfMask(0)
    }
}
impl core::fmt::Debug for ElsKdfMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsKdfMask")
            .field("kdf_mask", &self.kdf_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsKdfMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ElsKdfMask {{ kdf_mask: {=u32:?} }}", self.kdf_mask())
    }
}
#[doc = "ELS Lock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsLockCtrl(pub u32);
impl ElsLockCtrl {
    #[doc = "ELS Lock Control."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "ELS Lock Control."]
    #[inline(always)]
    pub const fn set_lock_ctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for ElsLockCtrl {
    #[inline(always)]
    fn default() -> ElsLockCtrl {
        ElsLockCtrl(0)
    }
}
impl core::fmt::Debug for ElsLockCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsLockCtrl")
            .field("lock_ctrl", &self.lock_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsLockCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ElsLockCtrl {{ lock_ctrl: {=u8:?} }}", self.lock_ctrl())
    }
}
#[doc = "ELS Lock Control DP."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsLockCtrlDp(pub u32);
impl ElsLockCtrlDp {
    #[doc = "Refer to ELS_LOCK_CTRL\\[1:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn lock_ctrl_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Refer to ELS_LOCK_CTRL\\[1:0\\]."]
    #[inline(always)]
    pub const fn set_lock_ctrl_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for ElsLockCtrlDp {
    #[inline(always)]
    fn default() -> ElsLockCtrlDp {
        ElsLockCtrlDp(0)
    }
}
impl core::fmt::Debug for ElsLockCtrlDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsLockCtrlDp")
            .field("lock_ctrl_dp", &self.lock_ctrl_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsLockCtrlDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsLockCtrlDp {{ lock_ctrl_dp: {=u8:?} }}",
            self.lock_ctrl_dp()
        )
    }
}
#[doc = "Life Cycle State Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcState(pub u32);
impl ElsOtpLcState {
    #[doc = "OTP life cycle state."]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state."]
    #[inline(always)]
    pub const fn set_otp_lc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcState {
    #[inline(always)]
    fn default() -> ElsOtpLcState {
        ElsOtpLcState(0)
    }
}
impl core::fmt::Debug for ElsOtpLcState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcState")
            .field("otp_lc_state", &self.otp_lc_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcState {{ otp_lc_state: {=u8:?} }}",
            self.otp_lc_state()
        )
    }
}
#[doc = "Life Cycle State Register (Duplicate)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsOtpLcStateDp(pub u32);
impl ElsOtpLcStateDp {
    #[doc = "OTP life cycle state."]
    #[must_use]
    #[inline(always)]
    pub const fn otp_lc_state_dp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "OTP life cycle state."]
    #[inline(always)]
    pub const fn set_otp_lc_state_dp(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for ElsOtpLcStateDp {
    #[inline(always)]
    fn default() -> ElsOtpLcStateDp {
        ElsOtpLcStateDp(0)
    }
}
impl core::fmt::Debug for ElsOtpLcStateDp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsOtpLcStateDp")
            .field("otp_lc_state_dp", &self.otp_lc_state_dp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsOtpLcStateDp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsOtpLcStateDp {{ otp_lc_state_dp: {=u8:?} }}",
            self.otp_lc_state_dp()
        )
    }
}
#[doc = "ELS Temporal State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ElsTemporalState(pub u32);
impl ElsTemporalState {
    #[doc = "Temporal state."]
    #[must_use]
    #[inline(always)]
    pub const fn temporal_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Temporal state."]
    #[inline(always)]
    pub const fn set_temporal_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for ElsTemporalState {
    #[inline(always)]
    fn default() -> ElsTemporalState {
        ElsTemporalState(0)
    }
}
impl core::fmt::Debug for ElsTemporalState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ElsTemporalState")
            .field("temporal_state", &self.temporal_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ElsTemporalState {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ElsTemporalState {{ temporal_state: {=u8:?} }}",
            self.temporal_state()
        )
    }
}
#[doc = "EMVSIM0 Function Clock Division."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim0clkdiv(pub u32);
impl Emvsim0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Emvsim0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Emvsim0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Emvsim0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Emvsim0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Emvsim0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Emvsim0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Emvsim0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Emvsim0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Emvsim0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Emvsim0clkdiv {
    #[inline(always)]
    fn default() -> Emvsim0clkdiv {
        Emvsim0clkdiv(0)
    }
}
impl core::fmt::Debug for Emvsim0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emvsim0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "EMVSIM0 Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim0clksel(pub u32);
impl Emvsim0clksel {
    #[doc = "Selects the EMVSIM0 function clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Emvsim0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Emvsim0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the EMVSIM0 function clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Emvsim0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Emvsim0clksel {
    #[inline(always)]
    fn default() -> Emvsim0clksel {
        Emvsim0clksel(0)
    }
}
impl core::fmt::Debug for Emvsim0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Emvsim0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "EMVSIM1 Function Clock Division."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim1clkdiv(pub u32);
impl Emvsim1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Emvsim1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Emvsim1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Emvsim1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Emvsim1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Emvsim1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Emvsim1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Emvsim1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Emvsim1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Emvsim1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Emvsim1clkdiv {
    #[inline(always)]
    fn default() -> Emvsim1clkdiv {
        Emvsim1clkdiv(0)
    }
}
impl core::fmt::Debug for Emvsim1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Emvsim1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "EMVSIM1 Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emvsim1clksel(pub u32);
impl Emvsim1clksel {
    #[doc = "Selects the EMVSIM1 function clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Emvsim1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Emvsim1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the EMVSIM1 function clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Emvsim1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Emvsim1clksel {
    #[inline(always)]
    fn default() -> Emvsim1clksel {
        Emvsim1clksel(0)
    }
}
impl core::fmt::Debug for Emvsim1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emvsim1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emvsim1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Emvsim1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Ethernet PHY Interface Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetPhyIntfSel(pub u32);
impl EnetPhyIntfSel {
    #[doc = "Selects the PHY interface."]
    #[must_use]
    #[inline(always)]
    pub const fn phy_sel(&self) -> PhySel {
        let val = (self.0 >> 2usize) & 0x01;
        PhySel::from_bits(val as u8)
    }
    #[doc = "Selects the PHY interface."]
    #[inline(always)]
    pub const fn set_phy_sel(&mut self, val: PhySel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for EnetPhyIntfSel {
    #[inline(always)]
    fn default() -> EnetPhyIntfSel {
        EnetPhyIntfSel(0)
    }
}
impl core::fmt::Debug for EnetPhyIntfSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetPhyIntfSel")
            .field("phy_sel", &self.phy_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetPhyIntfSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EnetPhyIntfSel {{ phy_sel: {:?} }}", self.phy_sel())
    }
}
#[doc = "Sideband Flow Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EnetSbdFlowCtrl(pub u32);
impl EnetSbdFlowCtrl {
    #[doc = "Sideband Flow Control for channel0."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Sideband Flow Control for channel0."]
    #[inline(always)]
    pub const fn set_sel_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sideband Flow Control for channel1."]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sideband Flow Control for channel1."]
    #[inline(always)]
    pub const fn set_sel_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for EnetSbdFlowCtrl {
    #[inline(always)]
    fn default() -> EnetSbdFlowCtrl {
        EnetSbdFlowCtrl(0)
    }
}
impl core::fmt::Debug for EnetSbdFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EnetSbdFlowCtrl")
            .field("sel_ch0", &self.sel_ch0())
            .field("sel_ch1", &self.sel_ch1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EnetSbdFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EnetSbdFlowCtrl {{ sel_ch0: {=bool:?}, sel_ch1: {=bool:?} }}",
            self.sel_ch0(),
            self.sel_ch1()
        )
    }
}
#[doc = "Ethernet PTP REF Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetptprefclkdiv(pub u32);
impl Enetptprefclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> EnetptprefclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        EnetptprefclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: EnetptprefclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> EnetptprefclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        EnetptprefclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: EnetptprefclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> EnetptprefclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        EnetptprefclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: EnetptprefclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Enetptprefclkdiv {
    #[inline(always)]
    fn default() -> Enetptprefclkdiv {
        Enetptprefclkdiv(0)
    }
}
impl core::fmt::Debug for Enetptprefclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetptprefclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetptprefclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enetptprefclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Ethernet PTP REF Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetptprefclksel(pub u32);
impl Enetptprefclksel {
    #[doc = "Selects the Ethernet PTP REF clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> EnetptprefclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        EnetptprefclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the Ethernet PTP REF clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: EnetptprefclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Enetptprefclksel {
    #[inline(always)]
    fn default() -> Enetptprefclksel {
        Enetptprefclksel(0)
    }
}
impl core::fmt::Debug for Enetptprefclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetptprefclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetptprefclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enetptprefclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Ethernet RMII Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetrmiiclkdiv(pub u32);
impl Enetrmiiclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> EnetrmiiclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        EnetrmiiclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: EnetrmiiclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> EnetrmiiclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        EnetrmiiclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: EnetrmiiclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> EnetrmiiclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        EnetrmiiclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: EnetrmiiclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Enetrmiiclkdiv {
    #[inline(always)]
    fn default() -> Enetrmiiclkdiv {
        Enetrmiiclkdiv(0)
    }
}
impl core::fmt::Debug for Enetrmiiclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetrmiiclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetrmiiclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Enetrmiiclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Ethernet RMII Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetrmiiclksel(pub u32);
impl Enetrmiiclksel {
    #[doc = "Selects the Ethernet RMII clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> EnetrmiiclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        EnetrmiiclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the Ethernet RMII clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: EnetrmiiclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Enetrmiiclksel {
    #[inline(always)]
    fn default() -> Enetrmiiclksel {
        Enetrmiiclksel(0)
    }
}
impl core::fmt::Debug for Enetrmiiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enetrmiiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enetrmiiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Enetrmiiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "ETB Counter Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbCounterCtrl(pub u32);
impl EtbCounterCtrl {
    #[doc = "Enables the ETB counter."]
    #[must_use]
    #[inline(always)]
    pub const fn cnten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the ETB counter."]
    #[inline(always)]
    pub const fn set_cnten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Response Type."]
    #[must_use]
    #[inline(always)]
    pub const fn rspt(&self) -> Rspt {
        let val = (self.0 >> 1usize) & 0x03;
        Rspt::from_bits(val as u8)
    }
    #[doc = "Response Type."]
    #[inline(always)]
    pub const fn set_rspt(&mut self, val: Rspt) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
    #[doc = "Reload request."]
    #[must_use]
    #[inline(always)]
    pub const fn rlrq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Reload request."]
    #[inline(always)]
    pub const fn set_rlrq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for EtbCounterCtrl {
    #[inline(always)]
    fn default() -> EtbCounterCtrl {
        EtbCounterCtrl(0)
    }
}
impl core::fmt::Debug for EtbCounterCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbCounterCtrl")
            .field("cnten", &self.cnten())
            .field("rspt", &self.rspt())
            .field("rlrq", &self.rlrq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbCounterCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EtbCounterCtrl {{ cnten: {=bool:?}, rspt: {:?}, rlrq: {=bool:?} }}",
            self.cnten(),
            self.rspt(),
            self.rlrq()
        )
    }
}
#[doc = "ETB Counter Reload Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbCounterReload(pub u32);
impl EtbCounterReload {
    #[doc = "Byte count reload value."]
    #[must_use]
    #[inline(always)]
    pub const fn reload(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Byte count reload value."]
    #[inline(always)]
    pub const fn set_reload(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for EtbCounterReload {
    #[inline(always)]
    fn default() -> EtbCounterReload {
        EtbCounterReload(0)
    }
}
impl core::fmt::Debug for EtbCounterReload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbCounterReload")
            .field("reload", &self.reload())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbCounterReload {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "EtbCounterReload {{ reload: {=u16:?} }}", self.reload())
    }
}
#[doc = "ETB Counter Value Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbCounterValue(pub u32);
impl EtbCounterValue {
    #[doc = "Byte count counter value."]
    #[must_use]
    #[inline(always)]
    pub const fn counter_value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Byte count counter value."]
    #[inline(always)]
    pub const fn set_counter_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for EtbCounterValue {
    #[inline(always)]
    fn default() -> EtbCounterValue {
        EtbCounterValue(0)
    }
}
impl core::fmt::Debug for EtbCounterValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbCounterValue")
            .field("counter_value", &self.counter_value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbCounterValue {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EtbCounterValue {{ counter_value: {=u16:?} }}",
            self.counter_value()
        )
    }
}
#[doc = "ETB Counter Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EtbStatus(pub u32);
impl EtbStatus {
    #[doc = "ETB Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn irq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ETB Interrupt."]
    #[inline(always)]
    pub const fn set_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ETB NMI."]
    #[must_use]
    #[inline(always)]
    pub const fn nmi(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "ETB NMI."]
    #[inline(always)]
    pub const fn set_nmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug halt request."]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_halt_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug halt request."]
    #[inline(always)]
    pub const fn set_dbg_halt_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for EtbStatus {
    #[inline(always)]
    fn default() -> EtbStatus {
        EtbStatus(0)
    }
}
impl core::fmt::Debug for EtbStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EtbStatus")
            .field("irq", &self.irq())
            .field("nmi", &self.nmi())
            .field("dbg_halt_req", &self.dbg_halt_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EtbStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EtbStatus {{ irq: {=bool:?}, nmi: {=bool:?}, dbg_halt_req: {=bool:?} }}",
            self.irq(),
            self.nmi(),
            self.dbg_halt_req()
        )
    }
}
#[doc = "EWM0 Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ewm0clksel(pub u32);
impl Ewm0clksel {
    #[doc = "Selects the EWM0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Ewm0clkselSel {
        let val = (self.0 >> 0usize) & 0x01;
        Ewm0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the EWM0 clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Ewm0clkselSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ewm0clksel {
    #[inline(always)]
    fn default() -> Ewm0clksel {
        Ewm0clksel(0)
    }
}
impl core::fmt::Debug for Ewm0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ewm0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ewm0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ewm0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "LP_FLEXCOMM Clock Source Select for Fractional Rate Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcclksel(pub u32);
impl Fcclksel {
    #[doc = "Selects the LP_FLEXCOMM clock source for Fractional Rate Divider."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> FcclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        FcclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the LP_FLEXCOMM clock source for Fractional Rate Divider."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: FcclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Fcclksel {
    #[inline(always)]
    fn default() -> Fcclksel {
        Fcclksel(0)
    }
}
impl core::fmt::Debug for Fcclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fcclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FlexSPI Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexSpiclkdiv(pub u32);
impl FlexSpiclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FlexSpiclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FlexSpiclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FlexSpiclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FlexSpiclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FlexSpiclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FlexSpiclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FlexSpiclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FlexSpiclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FlexSpiclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for FlexSpiclkdiv {
    #[inline(always)]
    fn default() -> FlexSpiclkdiv {
        FlexSpiclkdiv(0)
    }
}
impl core::fmt::Debug for FlexSpiclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexSpiclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexSpiclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FlexSpiclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FlexSPI Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlexSpiclksel(pub u32);
impl FlexSpiclksel {
    #[doc = "Selects the FlexSPI clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> FlexSpiclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        FlexSpiclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FlexSPI clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: FlexSpiclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for FlexSpiclksel {
    #[inline(always)]
    fn default() -> FlexSpiclksel {
        FlexSpiclksel(0)
    }
}
impl core::fmt::Debug for FlexSpiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlexSpiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlexSpiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FlexSpiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FLEXCAN0 Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan0clkdiv(pub u32);
impl Flexcan0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Flexcan0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Flexcan0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Flexcan0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Flexcan0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Flexcan0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Flexcan0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Flexcan0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Flexcan0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Flexcan0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexcan0clkdiv {
    #[inline(always)]
    fn default() -> Flexcan0clkdiv {
        Flexcan0clkdiv(0)
    }
}
impl core::fmt::Debug for Flexcan0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexcan0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN0 Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan0clksel(pub u32);
impl Flexcan0clksel {
    #[doc = "Selects the FLEXCAN0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Flexcan0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Flexcan0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FLEXCAN0 clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Flexcan0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexcan0clksel {
    #[inline(always)]
    fn default() -> Flexcan0clksel {
        Flexcan0clksel(0)
    }
}
impl core::fmt::Debug for Flexcan0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcan0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FLEXCAN1 Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan1clkdiv(pub u32);
impl Flexcan1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Flexcan1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Flexcan1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Flexcan1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Flexcan1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Flexcan1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Flexcan1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Flexcan1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Flexcan1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Flexcan1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexcan1clkdiv {
    #[inline(always)]
    fn default() -> Flexcan1clkdiv {
        Flexcan1clkdiv(0)
    }
}
impl core::fmt::Debug for Flexcan1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexcan1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXCAN1 Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcan1clksel(pub u32);
impl Flexcan1clksel {
    #[doc = "Selects the FLEXCAN1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Flexcan1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Flexcan1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FLEXCAN1 clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Flexcan1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexcan1clksel {
    #[inline(always)]
    fn default() -> Flexcan1clksel {
        Flexcan1clksel(0)
    }
}
impl core::fmt::Debug for Flexcan1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcan1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcan1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexcan1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "LP_FLEXCOMM Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcommclkdiv(pub u32);
impl Flexcommclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FlexcommclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FlexcommclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FlexcommclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FlexcommclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FlexcommclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FlexcommclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FlexcommclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FlexcommclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FlexcommclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexcommclkdiv {
    #[inline(always)]
    fn default() -> Flexcommclkdiv {
        Flexcommclkdiv(0)
    }
}
impl core::fmt::Debug for Flexcommclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexcommclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexcommclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexcommclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexioclkdiv(pub u32);
impl Flexioclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> FlexioclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        FlexioclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: FlexioclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FlexioclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FlexioclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FlexioclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FlexioclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FlexioclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FlexioclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexioclkdiv {
    #[inline(always)]
    fn default() -> Flexioclkdiv {
        Flexioclkdiv(0)
    }
}
impl core::fmt::Debug for Flexioclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexioclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexioclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Flexioclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "FLEXIO Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexioclksel(pub u32);
impl Flexioclksel {
    #[doc = "Selects the FLEXIO clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> FlexioclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        FlexioclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the FLEXIO clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: FlexioclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexioclksel {
    #[inline(always)]
    fn default() -> Flexioclksel {
        Flexioclksel(0)
    }
}
impl core::fmt::Debug for Flexioclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flexioclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flexioclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Flexioclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "FRO_HF_DIV Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frohfdiv(pub u32);
impl Frohfdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> FrohfdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        FrohfdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: FrohfdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> FrohfdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        FrohfdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: FrohfdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Frohfdiv {
    #[inline(always)]
    fn default() -> Frohfdiv {
        Frohfdiv(0)
    }
}
impl core::fmt::Debug for Frohfdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Frohfdiv")
            .field("div", &self.div())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Frohfdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Frohfdiv {{ div: {=u8:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "GDET Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GdetCtrl(pub u32);
impl GdetCtrl {
    #[doc = "Controls the GDET clean event counter."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_evtcnt_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Controls the GDET clean event counter."]
    #[inline(always)]
    pub const fn set_gdet_evtcnt_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears GDET error status."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_err_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears GDET error status."]
    #[inline(always)]
    pub const fn set_gdet_err_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "GDET isolation control."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_iso_sw(&self) -> GdetIsoSw {
        let val = (self.0 >> 2usize) & 0x03;
        GdetIsoSw::from_bits(val as u8)
    }
    #[doc = "GDET isolation control."]
    #[inline(always)]
    pub const fn set_gdet_iso_sw(&mut self, val: GdetIsoSw) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Event count value."]
    #[must_use]
    #[inline(always)]
    pub const fn event_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Event count value."]
    #[inline(always)]
    pub const fn set_event_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Positive glitch detected."]
    #[must_use]
    #[inline(always)]
    pub const fn pos_sync(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Positive glitch detected."]
    #[inline(always)]
    pub const fn set_pos_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Negative glitch detected."]
    #[must_use]
    #[inline(always)]
    pub const fn neg_sync(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Negative glitch detected."]
    #[inline(always)]
    pub const fn set_neg_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event counter cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn event_clr_flag(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event counter cleared."]
    #[inline(always)]
    pub const fn set_event_clr_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for GdetCtrl {
    #[inline(always)]
    fn default() -> GdetCtrl {
        GdetCtrl(0)
    }
}
impl core::fmt::Debug for GdetCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GdetCtrl")
            .field("gdet_evtcnt_clr", &self.gdet_evtcnt_clr())
            .field("gdet_err_clr", &self.gdet_err_clr())
            .field("gdet_iso_sw", &self.gdet_iso_sw())
            .field("event_cnt", &self.event_cnt())
            .field("pos_sync", &self.pos_sync())
            .field("neg_sync", &self.neg_sync())
            .field("event_clr_flag", &self.event_clr_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GdetCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GdetCtrl {{ gdet_evtcnt_clr: {=bool:?}, gdet_err_clr: {=bool:?}, gdet_iso_sw: {:?}, event_cnt: {=u8:?}, pos_sync: {=bool:?}, neg_sync: {=bool:?}, event_clr_flag: {=bool:?} }}",
            self.gdet_evtcnt_clr(),
            self.gdet_err_clr(),
            self.gdet_iso_sw(),
            self.event_cnt(),
            self.pos_sync(),
            self.neg_sync(),
            self.event_clr_flag()
        )
    }
}
#[doc = "Gray to Binary Converter Gray code_gray\\[31:0\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeLsb(pub u32);
impl GrayCodeLsb {
    #[doc = "Gray code \\[31:0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_31_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Gray code \\[31:0\\]."]
    #[inline(always)]
    pub const fn set_code_gray_31_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GrayCodeLsb {
    #[inline(always)]
    fn default() -> GrayCodeLsb {
        GrayCodeLsb(0)
    }
}
impl core::fmt::Debug for GrayCodeLsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeLsb")
            .field("code_gray_31_0", &self.code_gray_31_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeLsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeLsb {{ code_gray_31_0: {=u32:?} }}",
            self.code_gray_31_0()
        )
    }
}
#[doc = "Gray to Binary Converter Gray code_gray\\[41:32\\]."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GrayCodeMsb(pub u32);
impl GrayCodeMsb {
    #[doc = "Gray code \\[41:32\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn code_gray_41_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Gray code \\[41:32\\]."]
    #[inline(always)]
    pub const fn set_code_gray_41_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for GrayCodeMsb {
    #[inline(always)]
    fn default() -> GrayCodeMsb {
        GrayCodeMsb(0)
    }
}
impl core::fmt::Debug for GrayCodeMsb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GrayCodeMsb")
            .field("code_gray_41_32", &self.code_gray_41_32())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GrayCodeMsb {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "GrayCodeMsb {{ code_gray_41_32: {=u16:?} }}",
            self.code_gray_41_32()
        )
    }
}
#[doc = "I3C0 Functional Clock FCLK Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkdiv(pub u32);
impl I3c0fclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> I3c0fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        I3c0fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: I3c0fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> I3c0fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        I3c0fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: I3c0fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> I3c0fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        I3c0fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: I3c0fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclkdiv {
    #[inline(always)]
    fn default() -> I3c0fclkdiv {
        I3c0fclkdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0 FCLK Slow Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclksdiv(pub u32);
impl I3c0fclksdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> I3c0fclksdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        I3c0fclksdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: I3c0fclksdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> I3c0fclksdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        I3c0fclksdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: I3c0fclksdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> I3c0fclksdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        I3c0fclksdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: I3c0fclksdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclksdiv {
    #[inline(always)]
    fn default() -> I3c0fclksdiv {
        I3c0fclksdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclksdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclksdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclksdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclksdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0 Functional Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclksel(pub u32);
impl I3c0fclksel {
    #[doc = "Selects the I3C0 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> I3c0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        I3c0fclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the I3C0 clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: I3c0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclksel {
    #[inline(always)]
    fn default() -> I3c0fclksel {
        I3c0fclksel(0)
    }
}
impl core::fmt::Debug for I3c0fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C0 FCLK Slow Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkssel(pub u32);
impl I3c0fclkssel {
    #[doc = "Selects the I3C FCLK Slow clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> I3c0fclksselSel {
        let val = (self.0 >> 0usize) & 0x07;
        I3c0fclksselSel::from_bits(val as u8)
    }
    #[doc = "Selects the I3C FCLK Slow clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: I3c0fclksselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclkssel {
    #[inline(always)]
    fn default() -> I3c0fclkssel {
        I3c0fclkssel(0)
    }
}
impl core::fmt::Debug for I3c0fclkssel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkssel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkssel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclkssel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C0 FCLK_STC Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkstcdiv(pub u32);
impl I3c0fclkstcdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> I3c0fclkstcdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        I3c0fclkstcdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: I3c0fclkstcdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> I3c0fclkstcdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        I3c0fclkstcdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: I3c0fclkstcdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> I3c0fclkstcdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        I3c0fclkstcdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: I3c0fclkstcdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c0fclkstcdiv {
    #[inline(always)]
    fn default() -> I3c0fclkstcdiv {
        I3c0fclkstcdiv(0)
    }
}
impl core::fmt::Debug for I3c0fclkstcdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkstcdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkstcdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c0fclkstcdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C0 FCLK_STC Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c0fclkstcsel(pub u32);
impl I3c0fclkstcsel {
    #[doc = "Selects the I3C0 Time Control clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> I3c0fclkstcselSel {
        let val = (self.0 >> 0usize) & 0x07;
        I3c0fclkstcselSel::from_bits(val as u8)
    }
    #[doc = "Selects the I3C0 Time Control clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: I3c0fclkstcselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c0fclkstcsel {
    #[inline(always)]
    fn default() -> I3c0fclkstcsel {
        I3c0fclkstcsel(0)
    }
}
impl core::fmt::Debug for I3c0fclkstcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c0fclkstcsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c0fclkstcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c0fclkstcsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C1 Functional Clock FCLK Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkdiv(pub u32);
impl I3c1fclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> I3c1fclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        I3c1fclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: I3c1fclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> I3c1fclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        I3c1fclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: I3c1fclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> I3c1fclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        I3c1fclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: I3c1fclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c1fclkdiv {
    #[inline(always)]
    fn default() -> I3c1fclkdiv {
        I3c1fclkdiv(0)
    }
}
impl core::fmt::Debug for I3c1fclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c1fclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C1 FCLK Slow clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclksdiv(pub u32);
impl I3c1fclksdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> I3c1fclksdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        I3c1fclksdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: I3c1fclksdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> I3c1fclksdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        I3c1fclksdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: I3c1fclksdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> I3c1fclksdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        I3c1fclksdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: I3c1fclksdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c1fclksdiv {
    #[inline(always)]
    fn default() -> I3c1fclksdiv {
        I3c1fclksdiv(0)
    }
}
impl core::fmt::Debug for I3c1fclksdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclksdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclksdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c1fclksdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "I3C1 Functional Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclksel(pub u32);
impl I3c1fclksel {
    #[doc = "I3C1 clock select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> I3c1fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        I3c1fclkselSel::from_bits(val as u8)
    }
    #[doc = "I3C1 clock select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: I3c1fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c1fclksel {
    #[inline(always)]
    fn default() -> I3c1fclksel {
        I3c1fclksel(0)
    }
}
impl core::fmt::Debug for I3c1fclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c1fclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C1 FCLK Slow Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkssel(pub u32);
impl I3c1fclkssel {
    #[doc = "I3C1 FCLK Slow Clock Select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> I3c1fclksselSel {
        let val = (self.0 >> 0usize) & 0x07;
        I3c1fclksselSel::from_bits(val as u8)
    }
    #[doc = "I3C1 FCLK Slow Clock Select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: I3c1fclksselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c1fclkssel {
    #[inline(always)]
    fn default() -> I3c1fclkssel {
        I3c1fclkssel(0)
    }
}
impl core::fmt::Debug for I3c1fclkssel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkssel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkssel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c1fclkssel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "I3C1 FCLK_STC Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkstcdiv(pub u32);
impl I3c1fclkstcdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> I3c1fclkstcdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        I3c1fclkstcdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: I3c1fclkstcdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> I3c1fclkstcdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        I3c1fclkstcdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: I3c1fclkstcdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> I3c1fclkstcdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        I3c1fclkstcdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: I3c1fclkstcdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c1fclkstcdiv {
    #[inline(always)]
    fn default() -> I3c1fclkstcdiv {
        I3c1fclkstcdiv(0)
    }
}
impl core::fmt::Debug for I3c1fclkstcdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkstcdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkstcdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "I3c1fclkstcdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Selects the I3C1 Time Control clock."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c1fclkstcsel(pub u32);
impl I3c1fclkstcsel {
    #[doc = "I3C1 FCLK_STC clock select."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> I3c1fclkstcselSel {
        let val = (self.0 >> 0usize) & 0x07;
        I3c1fclkstcselSel::from_bits(val as u8)
    }
    #[doc = "I3C1 FCLK_STC clock select."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: I3c1fclkstcselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for I3c1fclkstcsel {
    #[inline(always)]
    fn default() -> I3c1fclkstcsel {
        I3c1fclkstcsel(0)
    }
}
impl core::fmt::Debug for I3c1fclkstcsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c1fclkstcsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c1fclkstcsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "I3c1fclkstcsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "JTAG Chip ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct JtagId(pub u32);
impl JtagId {
    #[doc = "Indicates the device ID."]
    #[must_use]
    #[inline(always)]
    pub const fn jtag_id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Indicates the device ID."]
    #[inline(always)]
    pub const fn set_jtag_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for JtagId {
    #[inline(always)]
    fn default() -> JtagId {
        JtagId(0)
    }
}
impl core::fmt::Debug for JtagId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JtagId")
            .field("jtag_id", &self.jtag_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for JtagId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "JtagId {{ jtag_id: {=u32:?} }}", self.jtag_id())
    }
}
#[doc = "Key Retain Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyRetainCtrl(pub u32);
impl KeyRetainCtrl {
    #[doc = "Indicates if the PUF key has been retained in the VBAT domain and has not been reset or otherwise invalidated by software."]
    #[must_use]
    #[inline(always)]
    pub const fn key_retain_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if the PUF key has been retained in the VBAT domain and has not been reset or otherwise invalidated by software."]
    #[inline(always)]
    pub const fn set_key_retain_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the successful completion of the key_save or key_load routine. Once set, to clear the key_retain_done flag, both key_save and key_load should be cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn key_retain_done(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the successful completion of the key_save or key_load routine. Once set, to clear the key_retain_done flag, both key_save and key_load should be cleared by software."]
    #[inline(always)]
    pub const fn set_key_retain_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time."]
    #[must_use]
    #[inline(always)]
    pub const fn key_save(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time."]
    #[inline(always)]
    pub const fn set_key_save(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time."]
    #[must_use]
    #[inline(always)]
    pub const fn key_load(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Do not set both KEY_SAVE and KEY_LOAD at the same time."]
    #[inline(always)]
    pub const fn set_key_load(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for KeyRetainCtrl {
    #[inline(always)]
    fn default() -> KeyRetainCtrl {
        KeyRetainCtrl(0)
    }
}
impl core::fmt::Debug for KeyRetainCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeyRetainCtrl")
            .field("key_retain_valid", &self.key_retain_valid())
            .field("key_retain_done", &self.key_retain_done())
            .field("key_save", &self.key_save())
            .field("key_load", &self.key_load())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeyRetainCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "KeyRetainCtrl {{ key_retain_valid: {=bool:?}, key_retain_done: {=bool:?}, key_save: {=bool:?}, key_load: {=bool:?} }}",
            self.key_retain_valid(),
            self.key_retain_done(),
            self.key_save(),
            self.key_load()
        )
    }
}
#[doc = "LPCAC Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpcacCtrl(pub u32);
impl LpcacCtrl {
    #[doc = "Disables/enables the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac(&self) -> DisLpcac {
        let val = (self.0 >> 0usize) & 0x01;
        DisLpcac::from_bits(val as u8)
    }
    #[doc = "Disables/enables the cache function."]
    #[inline(always)]
    pub const fn set_dis_lpcac(&mut self, val: DisLpcac) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the cache function."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_lpcac(&self) -> ClrLpcac {
        let val = (self.0 >> 1usize) & 0x01;
        ClrLpcac::from_bits(val as u8)
    }
    #[doc = "Clears the cache function."]
    #[inline(always)]
    pub const fn set_clr_lpcac(&mut self, val: ClrLpcac) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Forces no allocation."]
    #[must_use]
    #[inline(always)]
    pub const fn frc_no_alloc(&self) -> FrcNoAlloc {
        let val = (self.0 >> 2usize) & 0x01;
        FrcNoAlloc::from_bits(val as u8)
    }
    #[doc = "Forces no allocation."]
    #[inline(always)]
    pub const fn set_frc_no_alloc(&mut self, val: FrcNoAlloc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables parity miss."]
    #[must_use]
    #[inline(always)]
    pub const fn parity_miss_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enables parity miss."]
    #[inline(always)]
    pub const fn set_parity_miss_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Disable LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_dis_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[must_use]
    #[inline(always)]
    pub const fn lim_lpcac_wtbf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Limit LPCAC Write Through Buffer."]
    #[inline(always)]
    pub const fn set_lim_lpcac_wtbf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable parity error report."]
    #[must_use]
    #[inline(always)]
    pub const fn parity_fault_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable parity error report."]
    #[inline(always)]
    pub const fn set_parity_fault_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control."]
    #[must_use]
    #[inline(always)]
    pub const fn lpcac_xom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "LPCAC XOM(eXecute-Only-Memory) attribute control."]
    #[inline(always)]
    pub const fn set_lpcac_xom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for LpcacCtrl {
    #[inline(always)]
    fn default() -> LpcacCtrl {
        LpcacCtrl(0)
    }
}
impl core::fmt::Debug for LpcacCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpcacCtrl")
            .field("dis_lpcac", &self.dis_lpcac())
            .field("clr_lpcac", &self.clr_lpcac())
            .field("frc_no_alloc", &self.frc_no_alloc())
            .field("parity_miss_en", &self.parity_miss_en())
            .field("dis_lpcac_wtbf", &self.dis_lpcac_wtbf())
            .field("lim_lpcac_wtbf", &self.lim_lpcac_wtbf())
            .field("parity_fault_en", &self.parity_fault_en())
            .field("lpcac_xom", &self.lpcac_xom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpcacCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpcacCtrl {{ dis_lpcac: {:?}, clr_lpcac: {:?}, frc_no_alloc: {:?}, parity_miss_en: {=bool:?}, dis_lpcac_wtbf: {=bool:?}, lim_lpcac_wtbf: {=bool:?}, parity_fault_en: {=bool:?}, lpcac_xom: {=bool:?} }}",
            self.dis_lpcac(),
            self.clr_lpcac(),
            self.frc_no_alloc(),
            self.parity_miss_en(),
            self.dis_lpcac_wtbf(),
            self.lim_lpcac_wtbf(),
            self.parity_fault_en(),
            self.lpcac_xom()
        )
    }
}
#[doc = "MICFIL Clock Division."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Micfilfclkdiv(pub u32);
impl Micfilfclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> MicfilfclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        MicfilfclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: MicfilfclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> MicfilfclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        MicfilfclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: MicfilfclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> MicfilfclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        MicfilfclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: MicfilfclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Micfilfclkdiv {
    #[inline(always)]
    fn default() -> Micfilfclkdiv {
        Micfilfclkdiv(0)
    }
}
impl core::fmt::Debug for Micfilfclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Micfilfclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Micfilfclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Micfilfclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "MICFIL Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Micfilfclksel(pub u32);
impl Micfilfclksel {
    #[doc = "Selects the MICFIL clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> MicfilfclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        MicfilfclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the MICFIL clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: MicfilfclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Micfilfclksel {
    #[inline(always)]
    fn default() -> Micfilfclksel {
        Micfilfclksel(0)
    }
}
impl core::fmt::Debug for Micfilfclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Micfilfclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Micfilfclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Micfilfclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "NMI Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nmisrc(pub u32);
impl Nmisrc {
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU0, if enabled by NMIENCPU0."]
    #[inline(always)]
    pub const fn set_irqcpu0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU1, if enabled by NMIENCPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn irqcpu1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The IRQ number of the interrupt that acts as the Non-Maskable Interrupt (NMI) for CPU1, if enabled by NMIENCPU1."]
    #[inline(always)]
    pub const fn set_irqcpu1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu1(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU1."]
    #[inline(always)]
    pub const fn set_nmiencpu1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[must_use]
    #[inline(always)]
    pub const fn nmiencpu0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the Non-Maskable Interrupt (NMI) source selected by IRQCPU0."]
    #[inline(always)]
    pub const fn set_nmiencpu0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Nmisrc {
    #[inline(always)]
    fn default() -> Nmisrc {
        Nmisrc(0)
    }
}
impl core::fmt::Debug for Nmisrc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nmisrc")
            .field("irqcpu0", &self.irqcpu0())
            .field("irqcpu1", &self.irqcpu1())
            .field("nmiencpu1", &self.nmiencpu1())
            .field("nmiencpu0", &self.nmiencpu0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nmisrc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Nmisrc {{ irqcpu0: {=u8:?}, irqcpu1: {=u8:?}, nmiencpu1: {=bool:?}, nmiencpu0: {=bool:?} }}",
            self.irqcpu0(),
            self.irqcpu1(),
            self.nmiencpu1(),
            self.nmiencpu0()
        )
    }
}
#[doc = "NVM Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NvmCtrl(pub u32);
impl NvmCtrl {
    #[doc = "Flash speculation control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_spec(&self) -> DisFlashSpec {
        let val = (self.0 >> 0usize) & 0x01;
        DisFlashSpec::from_bits(val as u8)
    }
    #[doc = "Flash speculation control."]
    #[inline(always)]
    pub const fn set_dis_flash_spec(&mut self, val: DisFlashSpec) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Flash data speculation control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_data_spec(&self) -> DisDataSpec {
        let val = (self.0 >> 1usize) & 0x01;
        DisDataSpec::from_bits(val as u8)
    }
    #[doc = "Flash data speculation control."]
    #[inline(always)]
    pub const fn set_dis_data_spec(&mut self, val: DisDataSpec) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Flash cache control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_cache(&self) -> DisFlashCache {
        let val = (self.0 >> 2usize) & 0x01;
        DisFlashCache::from_bits(val as u8)
    }
    #[doc = "Flash cache control."]
    #[inline(always)]
    pub const fn set_dis_flash_cache(&mut self, val: DisFlashCache) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Flash instruction cache control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_inst(&self) -> DisFlashInst {
        let val = (self.0 >> 3usize) & 0x01;
        DisFlashInst::from_bits(val as u8)
    }
    #[doc = "Flash instruction cache control."]
    #[inline(always)]
    pub const fn set_dis_flash_inst(&mut self, val: DisFlashInst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Flash data cache control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_flash_data(&self) -> DisFlashData {
        let val = (self.0 >> 4usize) & 0x01;
        DisFlashData::from_bits(val as u8)
    }
    #[doc = "Flash data cache control."]
    #[inline(always)]
    pub const fn set_dis_flash_data(&mut self, val: DisFlashData) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Clear flash cache control."]
    #[must_use]
    #[inline(always)]
    pub const fn clr_flash_cache(&self) -> ClrFlashCache {
        let val = (self.0 >> 5usize) & 0x01;
        ClrFlashCache::from_bits(val as u8)
    }
    #[doc = "Clear flash cache control."]
    #[inline(always)]
    pub const fn set_clr_flash_cache(&mut self, val: ClrFlashCache) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLASH stall on busy control."]
    #[must_use]
    #[inline(always)]
    pub const fn flash_stall_en(&self) -> FlashStallEn {
        let val = (self.0 >> 10usize) & 0x01;
        FlashStallEn::from_bits(val as u8)
    }
    #[doc = "FLASH stall on busy control."]
    #[inline(always)]
    pub const fn set_flash_stall_en(&mut self, val: FlashStallEn) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bus error on instruction multi-bit ECC error control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_inst(&self) -> DisMbeccErrInst {
        let val = (self.0 >> 16usize) & 0x01;
        DisMbeccErrInst::from_bits(val as u8)
    }
    #[doc = "Bus error on instruction multi-bit ECC error control."]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_inst(&mut self, val: DisMbeccErrInst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Bus error on data multi-bit ECC error control."]
    #[must_use]
    #[inline(always)]
    pub const fn dis_mbecc_err_data(&self) -> DisMbeccErrData {
        let val = (self.0 >> 17usize) & 0x01;
        DisMbeccErrData::from_bits(val as u8)
    }
    #[doc = "Bus error on data multi-bit ECC error control."]
    #[inline(always)]
    pub const fn set_dis_mbecc_err_data(&mut self, val: DisMbeccErrData) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
}
impl Default for NvmCtrl {
    #[inline(always)]
    fn default() -> NvmCtrl {
        NvmCtrl(0)
    }
}
impl core::fmt::Debug for NvmCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NvmCtrl")
            .field("dis_flash_spec", &self.dis_flash_spec())
            .field("dis_data_spec", &self.dis_data_spec())
            .field("dis_flash_cache", &self.dis_flash_cache())
            .field("dis_flash_inst", &self.dis_flash_inst())
            .field("dis_flash_data", &self.dis_flash_data())
            .field("clr_flash_cache", &self.clr_flash_cache())
            .field("flash_stall_en", &self.flash_stall_en())
            .field("dis_mbecc_err_inst", &self.dis_mbecc_err_inst())
            .field("dis_mbecc_err_data", &self.dis_mbecc_err_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NvmCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NvmCtrl {{ dis_flash_spec: {:?}, dis_data_spec: {:?}, dis_flash_cache: {:?}, dis_flash_inst: {:?}, dis_flash_data: {:?}, clr_flash_cache: {:?}, flash_stall_en: {:?}, dis_mbecc_err_inst: {:?}, dis_mbecc_err_data: {:?} }}",
            self.dis_flash_spec(),
            self.dis_data_spec(),
            self.dis_flash_cache(),
            self.dis_flash_inst(),
            self.dis_flash_data(),
            self.clr_flash_cache(),
            self.flash_stall_en(),
            self.dis_mbecc_err_inst(),
            self.dis_mbecc_err_data()
        )
    }
}
#[doc = "OSTIMER Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostimerclksel(pub u32);
impl Ostimerclksel {
    #[doc = "Selects the OS Event Timer clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> OstimerclkselSel {
        let val = (self.0 >> 0usize) & 0x03;
        OstimerclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the OS Event Timer clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: OstimerclkselSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Ostimerclksel {
    #[inline(always)]
    fn default() -> Ostimerclksel {
        Ostimerclksel(0)
    }
}
impl core::fmt::Debug for Ostimerclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ostimerclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ostimerclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ostimerclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "PLL1 Clock 0 Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clk0div(pub u32);
impl Pll1clk0div {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Pll1clk0divReset {
        let val = (self.0 >> 29usize) & 0x01;
        Pll1clk0divReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Pll1clk0divReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Pll1clk0divHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Pll1clk0divHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Pll1clk0divHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Pll1clk0divUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Pll1clk0divUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Pll1clk0divUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clk0div {
    #[inline(always)]
    fn default() -> Pll1clk0div {
        Pll1clk0div(0)
    }
}
impl core::fmt::Debug for Pll1clk0div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clk0div")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clk0div {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clk0div {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "PLL1 Clock 1 Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll1clk1div(pub u32);
impl Pll1clk1div {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Pll1clk1divReset {
        let val = (self.0 >> 29usize) & 0x01;
        Pll1clk1divReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Pll1clk1divReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Pll1clk1divHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Pll1clk1divHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Pll1clk1divHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Pll1clk1divUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Pll1clk1divUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Pll1clk1divUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pll1clk1div {
    #[inline(always)]
    fn default() -> Pll1clk1div {
        Pll1clk1div(0)
    }
}
impl core::fmt::Debug for Pll1clk1div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pll1clk1div")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pll1clk1div {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pll1clk1div {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "PLL Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllclkdiv(pub u32);
impl Pllclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> PllclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        PllclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: PllclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> PllclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        PllclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: PllclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> PllclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        PllclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: PllclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pllclkdiv {
    #[inline(always)]
    fn default() -> Pllclkdiv {
        Pllclkdiv(0)
    }
}
impl core::fmt::Debug for Pllclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pllclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pllclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pllclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "PLL Clock Divider Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pllclkdivsel(pub u32);
impl Pllclkdivsel {
    #[doc = "Selects the PLL Clock Divider source clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> PllclkdivselSel {
        let val = (self.0 >> 0usize) & 0x07;
        PllclkdivselSel::from_bits(val as u8)
    }
    #[doc = "Selects the PLL Clock Divider source clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: PllclkdivselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Pllclkdivsel {
    #[inline(always)]
    fn default() -> Pllclkdivsel {
        Pllclkdivsel(0)
    }
}
impl core::fmt::Debug for Pllclkdivsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pllclkdivsel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pllclkdivsel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pllclkdivsel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "Peripheral Reset Control 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl0(pub u32);
impl Presetctrl0 {
    #[doc = "Flash management unit reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn fmu_rst(&self) -> FmuRst {
        let val = (self.0 >> 9usize) & 0x01;
        FmuRst::from_bits(val as u8)
    }
    #[doc = "Flash management unit reset control."]
    #[inline(always)]
    pub const fn set_fmu_rst(&mut self, val: FmuRst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FlexSPI reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn flexspi_rst(&self) -> FlexspiRst {
        let val = (self.0 >> 11usize) & 0x01;
        FlexspiRst::from_bits(val as u8)
    }
    #[doc = "FlexSPI reset control."]
    #[inline(always)]
    pub const fn set_flexspi_rst(&mut self, val: FlexspiRst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "INPUTMUX reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn mux_rst(&self) -> MuxRst {
        let val = (self.0 >> 12usize) & 0x01;
        MuxRst::from_bits(val as u8)
    }
    #[doc = "INPUTMUX reset control."]
    #[inline(always)]
    pub const fn set_mux_rst(&mut self, val: MuxRst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "PORT controller reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn port_rst(&self, n: usize) -> PortRst {
        assert!(n < 5usize);
        let offs = 13usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        PortRst::from_bits(val as u8)
    }
    #[doc = "PORT controller reset control."]
    #[inline(always)]
    pub const fn set_port_rst(&mut self, n: usize, val: PortRst) {
        assert!(n < 5usize);
        let offs = 13usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "GPIO reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_rst(&self, n: usize) -> GpioRst {
        assert!(n < 5usize);
        let offs = 19usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        GpioRst::from_bits(val as u8)
    }
    #[doc = "GPIO reset control."]
    #[inline(always)]
    pub const fn set_gpio_rst(&mut self, n: usize, val: GpioRst) {
        assert!(n < 5usize);
        let offs = 19usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "PINT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pint_rst(&self) -> PintRst {
        let val = (self.0 >> 25usize) & 0x01;
        PintRst::from_bits(val as u8)
    }
    #[doc = "PINT reset control."]
    #[inline(always)]
    pub const fn set_pint_rst(&mut self, val: PintRst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMA0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dma0_rst(&self) -> Dma0Rst {
        let val = (self.0 >> 26usize) & 0x01;
        Dma0Rst::from_bits(val as u8)
    }
    #[doc = "DMA0 reset control."]
    #[inline(always)]
    pub const fn set_dma0_rst(&mut self, val: Dma0Rst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "CRC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn crc_rst(&self) -> CrcRst {
        let val = (self.0 >> 27usize) & 0x01;
        CrcRst::from_bits(val as u8)
    }
    #[doc = "CRC reset control."]
    #[inline(always)]
    pub const fn set_crc_rst(&mut self, val: CrcRst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Inter-CPU communication Mailbox reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn mailbox_rst(&self) -> MailboxRst {
        let val = (self.0 >> 31usize) & 0x01;
        MailboxRst::from_bits(val as u8)
    }
    #[doc = "Inter-CPU communication Mailbox reset control."]
    #[inline(always)]
    pub const fn set_mailbox_rst(&mut self, val: MailboxRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Presetctrl0 {
    #[inline(always)]
    fn default() -> Presetctrl0 {
        Presetctrl0(0)
    }
}
impl core::fmt::Debug for Presetctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl0")
            .field("fmu_rst", &self.fmu_rst())
            .field("flexspi_rst", &self.flexspi_rst())
            .field("mux_rst", &self.mux_rst())
            .field("port_rst[0]", &self.port_rst(0usize))
            .field("port_rst[1]", &self.port_rst(1usize))
            .field("port_rst[2]", &self.port_rst(2usize))
            .field("port_rst[3]", &self.port_rst(3usize))
            .field("port_rst[4]", &self.port_rst(4usize))
            .field("gpio_rst[0]", &self.gpio_rst(0usize))
            .field("gpio_rst[1]", &self.gpio_rst(1usize))
            .field("gpio_rst[2]", &self.gpio_rst(2usize))
            .field("gpio_rst[3]", &self.gpio_rst(3usize))
            .field("gpio_rst[4]", &self.gpio_rst(4usize))
            .field("pint_rst", &self.pint_rst())
            .field("dma0_rst", &self.dma0_rst())
            .field("crc_rst", &self.crc_rst())
            .field("mailbox_rst", &self.mailbox_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl0 {{ fmu_rst: {:?}, flexspi_rst: {:?}, mux_rst: {:?}, port_rst[0]: {:?}, port_rst[1]: {:?}, port_rst[2]: {:?}, port_rst[3]: {:?}, port_rst[4]: {:?}, gpio_rst[0]: {:?}, gpio_rst[1]: {:?}, gpio_rst[2]: {:?}, gpio_rst[3]: {:?}, gpio_rst[4]: {:?}, pint_rst: {:?}, dma0_rst: {:?}, crc_rst: {:?}, mailbox_rst: {:?} }}",
            self.fmu_rst(),
            self.flexspi_rst(),
            self.mux_rst(),
            self.port_rst(0usize),
            self.port_rst(1usize),
            self.port_rst(2usize),
            self.port_rst(3usize),
            self.port_rst(4usize),
            self.gpio_rst(0usize),
            self.gpio_rst(1usize),
            self.gpio_rst(2usize),
            self.gpio_rst(3usize),
            self.gpio_rst(4usize),
            self.pint_rst(),
            self.dma0_rst(),
            self.crc_rst(),
            self.mailbox_rst()
        )
    }
}
#[doc = "Peripheral Reset Control 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl1(pub u32);
impl Presetctrl1 {
    #[doc = "MRT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn mrt_rst(&self) -> MrtRst {
        let val = (self.0 >> 0usize) & 0x01;
        MrtRst::from_bits(val as u8)
    }
    #[doc = "MRT reset control."]
    #[inline(always)]
    pub const fn set_mrt_rst(&mut self, val: MrtRst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "OS Event Timer reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn ostimer_rst(&self) -> OstimerRst {
        let val = (self.0 >> 1usize) & 0x01;
        OstimerRst::from_bits(val as u8)
    }
    #[doc = "OS Event Timer reset control."]
    #[inline(always)]
    pub const fn set_ostimer_rst(&mut self, val: OstimerRst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SCT reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sct_rst(&self) -> SctRst {
        let val = (self.0 >> 2usize) & 0x01;
        SctRst::from_bits(val as u8)
    }
    #[doc = "SCT reset control."]
    #[inline(always)]
    pub const fn set_sct_rst(&mut self, val: SctRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "ADC0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn adc0_rst(&self) -> Adc0Rst {
        let val = (self.0 >> 3usize) & 0x01;
        Adc0Rst::from_bits(val as u8)
    }
    #[doc = "ADC0 reset control."]
    #[inline(always)]
    pub const fn set_adc0_rst(&mut self, val: Adc0Rst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "ADC1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1_rst(&self) -> Adc1Rst {
        let val = (self.0 >> 4usize) & 0x01;
        Adc1Rst::from_bits(val as u8)
    }
    #[doc = "ADC1 reset control."]
    #[inline(always)]
    pub const fn set_adc1_rst(&mut self, val: Adc1Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DAC0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dac0_rst(&self) -> Dac0Rst {
        let val = (self.0 >> 5usize) & 0x01;
        Dac0Rst::from_bits(val as u8)
    }
    #[doc = "DAC0 reset control."]
    #[inline(always)]
    pub const fn set_dac0_rst(&mut self, val: Dac0Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "RTC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn rtc_rst(&self) -> RtcRst {
        let val = (self.0 >> 6usize) & 0x01;
        RtcRst::from_bits(val as u8)
    }
    #[doc = "RTC reset control."]
    #[inline(always)]
    pub const fn set_rtc_rst(&mut self, val: RtcRst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "EVSIM0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn evsim0_rst(&self) -> Evsim0Rst {
        let val = (self.0 >> 8usize) & 0x01;
        Evsim0Rst::from_bits(val as u8)
    }
    #[doc = "EVSIM0 reset control."]
    #[inline(always)]
    pub const fn set_evsim0_rst(&mut self, val: Evsim0Rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "EVSIM1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn evsim1_rst(&self) -> Evsim1Rst {
        let val = (self.0 >> 9usize) & 0x01;
        Evsim1Rst::from_bits(val as u8)
    }
    #[doc = "EVSIM1 reset control."]
    #[inline(always)]
    pub const fn set_evsim1_rst(&mut self, val: Evsim1Rst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "UTICK reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn utick_rst(&self) -> UtickRst {
        let val = (self.0 >> 10usize) & 0x01;
        UtickRst::from_bits(val as u8)
    }
    #[doc = "UTICK reset control."]
    #[inline(always)]
    pub const fn set_utick_rst(&mut self, val: UtickRst) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "LP_FLEXCOMM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn fc_rst(&self, n: usize) -> FcRst {
        assert!(n < 10usize);
        let offs = 11usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        FcRst::from_bits(val as u8)
    }
    #[doc = "LP_FLEXCOMM reset control."]
    #[inline(always)]
    pub const fn set_fc_rst(&mut self, n: usize, val: FcRst) {
        assert!(n < 10usize);
        let offs = 11usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "MICFIL reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn micfil_rst(&self) -> MicfilRst {
        let val = (self.0 >> 21usize) & 0x01;
        MicfilRst::from_bits(val as u8)
    }
    #[doc = "MICFIL reset control."]
    #[inline(always)]
    pub const fn set_micfil_rst(&mut self, val: MicfilRst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "CTIMER2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer2_rst(&self) -> Timer2Rst {
        let val = (self.0 >> 22usize) & 0x01;
        Timer2Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER2 reset control."]
    #[inline(always)]
    pub const fn set_timer2_rst(&mut self, val: Timer2Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USB FS DCD reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs_dcd_rst(&self) -> Usb0FsDcdRst {
        let val = (self.0 >> 24usize) & 0x01;
        Usb0FsDcdRst::from_bits(val as u8)
    }
    #[doc = "USB FS DCD reset control."]
    #[inline(always)]
    pub const fn set_usb0_fs_dcd_rst(&mut self, val: Usb0FsDcdRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "USB FS reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb0_fs_rst(&self) -> Usb0FsRst {
        let val = (self.0 >> 25usize) & 0x01;
        Usb0FsRst::from_bits(val as u8)
    }
    #[doc = "USB FS reset control."]
    #[inline(always)]
    pub const fn set_usb0_fs_rst(&mut self, val: Usb0FsRst) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "CTIMER0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer0_rst(&self) -> Timer0Rst {
        let val = (self.0 >> 26usize) & 0x01;
        Timer0Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER0 reset control."]
    #[inline(always)]
    pub const fn set_timer0_rst(&mut self, val: Timer0Rst) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "CTIMER1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer1_rst(&self) -> Timer1Rst {
        let val = (self.0 >> 27usize) & 0x01;
        Timer1Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER1 reset control."]
    #[inline(always)]
    pub const fn set_timer1_rst(&mut self, val: Timer1Rst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "SmartDMA reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn smart_dma_rst(&self) -> SmartDmaRst {
        let val = (self.0 >> 31usize) & 0x01;
        SmartDmaRst::from_bits(val as u8)
    }
    #[doc = "SmartDMA reset control."]
    #[inline(always)]
    pub const fn set_smart_dma_rst(&mut self, val: SmartDmaRst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Presetctrl1 {
    #[inline(always)]
    fn default() -> Presetctrl1 {
        Presetctrl1(0)
    }
}
impl core::fmt::Debug for Presetctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl1")
            .field("mrt_rst", &self.mrt_rst())
            .field("ostimer_rst", &self.ostimer_rst())
            .field("sct_rst", &self.sct_rst())
            .field("adc0_rst", &self.adc0_rst())
            .field("adc1_rst", &self.adc1_rst())
            .field("dac0_rst", &self.dac0_rst())
            .field("rtc_rst", &self.rtc_rst())
            .field("evsim0_rst", &self.evsim0_rst())
            .field("evsim1_rst", &self.evsim1_rst())
            .field("utick_rst", &self.utick_rst())
            .field("fc_rst[0]", &self.fc_rst(0usize))
            .field("fc_rst[1]", &self.fc_rst(1usize))
            .field("fc_rst[2]", &self.fc_rst(2usize))
            .field("fc_rst[3]", &self.fc_rst(3usize))
            .field("fc_rst[4]", &self.fc_rst(4usize))
            .field("fc_rst[5]", &self.fc_rst(5usize))
            .field("fc_rst[6]", &self.fc_rst(6usize))
            .field("fc_rst[7]", &self.fc_rst(7usize))
            .field("fc_rst[8]", &self.fc_rst(8usize))
            .field("fc_rst[9]", &self.fc_rst(9usize))
            .field("micfil_rst", &self.micfil_rst())
            .field("timer2_rst", &self.timer2_rst())
            .field("usb0_fs_dcd_rst", &self.usb0_fs_dcd_rst())
            .field("usb0_fs_rst", &self.usb0_fs_rst())
            .field("timer0_rst", &self.timer0_rst())
            .field("timer1_rst", &self.timer1_rst())
            .field("smart_dma_rst", &self.smart_dma_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl1 {{ mrt_rst: {:?}, ostimer_rst: {:?}, sct_rst: {:?}, adc0_rst: {:?}, adc1_rst: {:?}, dac0_rst: {:?}, rtc_rst: {:?}, evsim0_rst: {:?}, evsim1_rst: {:?}, utick_rst: {:?}, fc_rst[0]: {:?}, fc_rst[1]: {:?}, fc_rst[2]: {:?}, fc_rst[3]: {:?}, fc_rst[4]: {:?}, fc_rst[5]: {:?}, fc_rst[6]: {:?}, fc_rst[7]: {:?}, fc_rst[8]: {:?}, fc_rst[9]: {:?}, micfil_rst: {:?}, timer2_rst: {:?}, usb0_fs_dcd_rst: {:?}, usb0_fs_rst: {:?}, timer0_rst: {:?}, timer1_rst: {:?}, smart_dma_rst: {:?} }}",
            self.mrt_rst(),
            self.ostimer_rst(),
            self.sct_rst(),
            self.adc0_rst(),
            self.adc1_rst(),
            self.dac0_rst(),
            self.rtc_rst(),
            self.evsim0_rst(),
            self.evsim1_rst(),
            self.utick_rst(),
            self.fc_rst(0usize),
            self.fc_rst(1usize),
            self.fc_rst(2usize),
            self.fc_rst(3usize),
            self.fc_rst(4usize),
            self.fc_rst(5usize),
            self.fc_rst(6usize),
            self.fc_rst(7usize),
            self.fc_rst(8usize),
            self.fc_rst(9usize),
            self.micfil_rst(),
            self.timer2_rst(),
            self.usb0_fs_dcd_rst(),
            self.usb0_fs_rst(),
            self.timer0_rst(),
            self.timer1_rst(),
            self.smart_dma_rst()
        )
    }
}
#[doc = "Peripheral Reset Control 2."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl2(pub u32);
impl Presetctrl2 {
    #[doc = "DMA1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dma1_rst(&self) -> Dma1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        Dma1Rst::from_bits(val as u8)
    }
    #[doc = "DMA1 reset control."]
    #[inline(always)]
    pub const fn set_dma1_rst(&mut self, val: Dma1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Ethernet reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn enet_rst(&self) -> EnetRst {
        let val = (self.0 >> 2usize) & 0x01;
        EnetRst::from_bits(val as u8)
    }
    #[doc = "Ethernet reset control."]
    #[inline(always)]
    pub const fn set_enet_rst(&mut self, val: EnetRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "uSDHC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usdhc_rst(&self) -> UsdhcRst {
        let val = (self.0 >> 3usize) & 0x01;
        UsdhcRst::from_bits(val as u8)
    }
    #[doc = "uSDHC reset control."]
    #[inline(always)]
    pub const fn set_usdhc_rst(&mut self, val: UsdhcRst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXIO reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn flexio_rst(&self) -> FlexioRst {
        let val = (self.0 >> 4usize) & 0x01;
        FlexioRst::from_bits(val as u8)
    }
    #[doc = "FLEXIO reset control."]
    #[inline(always)]
    pub const fn set_flexio_rst(&mut self, val: FlexioRst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "SAI0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sai0_rst(&self) -> Sai0Rst {
        let val = (self.0 >> 5usize) & 0x01;
        Sai0Rst::from_bits(val as u8)
    }
    #[doc = "SAI0 reset control."]
    #[inline(always)]
    pub const fn set_sai0_rst(&mut self, val: Sai0Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "SAI1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sai1_rst(&self) -> Sai1Rst {
        let val = (self.0 >> 6usize) & 0x01;
        Sai1Rst::from_bits(val as u8)
    }
    #[doc = "SAI1 reset control."]
    #[inline(always)]
    pub const fn set_sai1_rst(&mut self, val: Sai1Rst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "TRO reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn tro_rst(&self) -> TroRst {
        let val = (self.0 >> 7usize) & 0x01;
        TroRst::from_bits(val as u8)
    }
    #[doc = "TRO reset control."]
    #[inline(always)]
    pub const fn set_tro_rst(&mut self, val: TroRst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FREQME reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn freqme_rst(&self) -> FreqmeRst {
        let val = (self.0 >> 8usize) & 0x01;
        FreqmeRst::from_bits(val as u8)
    }
    #[doc = "FREQME reset control."]
    #[inline(always)]
    pub const fn set_freqme_rst(&mut self, val: FreqmeRst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "TRNG reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_rst(&self) -> TrngRst {
        let val = (self.0 >> 13usize) & 0x01;
        TrngRst::from_bits(val as u8)
    }
    #[doc = "TRNG reset control."]
    #[inline(always)]
    pub const fn set_trng_rst(&mut self, val: TrngRst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "CAN0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan0_rst(&self) -> Flexcan0Rst {
        let val = (self.0 >> 14usize) & 0x01;
        Flexcan0Rst::from_bits(val as u8)
    }
    #[doc = "CAN0 reset control."]
    #[inline(always)]
    pub const fn set_flexcan0_rst(&mut self, val: Flexcan0Rst) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "CAN1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn flexcan1_rst(&self) -> Flexcan1Rst {
        let val = (self.0 >> 15usize) & 0x01;
        Flexcan1Rst::from_bits(val as u8)
    }
    #[doc = "CAN1 reset control."]
    #[inline(always)]
    pub const fn set_flexcan1_rst(&mut self, val: Flexcan1Rst) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "USB HS reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_rst(&self) -> UsbHsRst {
        let val = (self.0 >> 16usize) & 0x01;
        UsbHsRst::from_bits(val as u8)
    }
    #[doc = "USB HS reset control."]
    #[inline(always)]
    pub const fn set_usb_hs_rst(&mut self, val: UsbHsRst) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "USB HS PHY reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn usb_hs_phy_rst(&self) -> UsbHsPhyRst {
        let val = (self.0 >> 17usize) & 0x01;
        UsbHsPhyRst::from_bits(val as u8)
    }
    #[doc = "USB HS PHY reset control."]
    #[inline(always)]
    pub const fn set_usb_hs_phy_rst(&mut self, val: UsbHsPhyRst) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "PowerQuad reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pq_rst(&self) -> PqRst {
        let val = (self.0 >> 19usize) & 0x01;
        PqRst::from_bits(val as u8)
    }
    #[doc = "PowerQuad reset control."]
    #[inline(always)]
    pub const fn set_pq_rst(&mut self, val: PqRst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "PLU reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn plu_rst(&self) -> PluRst {
        let val = (self.0 >> 20usize) & 0x01;
        PluRst::from_bits(val as u8)
    }
    #[doc = "PLU reset control."]
    #[inline(always)]
    pub const fn set_plu_rst(&mut self, val: PluRst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "CTIMER3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer3_rst(&self) -> Timer3Rst {
        let val = (self.0 >> 21usize) & 0x01;
        Timer3Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER3 reset control."]
    #[inline(always)]
    pub const fn set_timer3_rst(&mut self, val: Timer3Rst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "CTIMER4 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn timer4_rst(&self) -> Timer4Rst {
        let val = (self.0 >> 22usize) & 0x01;
        Timer4Rst::from_bits(val as u8)
    }
    #[doc = "CTIMER4 reset control."]
    #[inline(always)]
    pub const fn set_timer4_rst(&mut self, val: Timer4Rst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "PUF reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn puf_rst(&self) -> PufRst {
        let val = (self.0 >> 23usize) & 0x01;
        PufRst::from_bits(val as u8)
    }
    #[doc = "PUF reset control."]
    #[inline(always)]
    pub const fn set_puf_rst(&mut self, val: PufRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "PKC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pkc_rst(&self) -> PkcRst {
        let val = (self.0 >> 24usize) & 0x01;
        PkcRst::from_bits(val as u8)
    }
    #[doc = "PKC reset control."]
    #[inline(always)]
    pub const fn set_pkc_rst(&mut self, val: PkcRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SM3 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sm3_rst(&self) -> Sm3Rst {
        let val = (self.0 >> 30usize) & 0x01;
        Sm3Rst::from_bits(val as u8)
    }
    #[doc = "SM3 reset control."]
    #[inline(always)]
    pub const fn set_sm3_rst(&mut self, val: Sm3Rst) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Presetctrl2 {
    #[inline(always)]
    fn default() -> Presetctrl2 {
        Presetctrl2(0)
    }
}
impl core::fmt::Debug for Presetctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl2")
            .field("dma1_rst", &self.dma1_rst())
            .field("enet_rst", &self.enet_rst())
            .field("usdhc_rst", &self.usdhc_rst())
            .field("flexio_rst", &self.flexio_rst())
            .field("sai0_rst", &self.sai0_rst())
            .field("sai1_rst", &self.sai1_rst())
            .field("tro_rst", &self.tro_rst())
            .field("freqme_rst", &self.freqme_rst())
            .field("trng_rst", &self.trng_rst())
            .field("flexcan0_rst", &self.flexcan0_rst())
            .field("flexcan1_rst", &self.flexcan1_rst())
            .field("usb_hs_rst", &self.usb_hs_rst())
            .field("usb_hs_phy_rst", &self.usb_hs_phy_rst())
            .field("pq_rst", &self.pq_rst())
            .field("plu_rst", &self.plu_rst())
            .field("timer3_rst", &self.timer3_rst())
            .field("timer4_rst", &self.timer4_rst())
            .field("puf_rst", &self.puf_rst())
            .field("pkc_rst", &self.pkc_rst())
            .field("sm3_rst", &self.sm3_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl2 {{ dma1_rst: {:?}, enet_rst: {:?}, usdhc_rst: {:?}, flexio_rst: {:?}, sai0_rst: {:?}, sai1_rst: {:?}, tro_rst: {:?}, freqme_rst: {:?}, trng_rst: {:?}, flexcan0_rst: {:?}, flexcan1_rst: {:?}, usb_hs_rst: {:?}, usb_hs_phy_rst: {:?}, pq_rst: {:?}, plu_rst: {:?}, timer3_rst: {:?}, timer4_rst: {:?}, puf_rst: {:?}, pkc_rst: {:?}, sm3_rst: {:?} }}",
            self.dma1_rst(),
            self.enet_rst(),
            self.usdhc_rst(),
            self.flexio_rst(),
            self.sai0_rst(),
            self.sai1_rst(),
            self.tro_rst(),
            self.freqme_rst(),
            self.trng_rst(),
            self.flexcan0_rst(),
            self.flexcan1_rst(),
            self.usb_hs_rst(),
            self.usb_hs_phy_rst(),
            self.pq_rst(),
            self.plu_rst(),
            self.timer3_rst(),
            self.timer4_rst(),
            self.puf_rst(),
            self.pkc_rst(),
            self.sm3_rst()
        )
    }
}
#[doc = "Peripheral Reset Control 3."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrl3(pub u32);
impl Presetctrl3 {
    #[doc = "I3C0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c0_rst(&self) -> I3c0Rst {
        let val = (self.0 >> 0usize) & 0x01;
        I3c0Rst::from_bits(val as u8)
    }
    #[doc = "I3C0 reset control."]
    #[inline(always)]
    pub const fn set_i3c0_rst(&mut self, val: I3c0Rst) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "I3C1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn i3c1_rst(&self) -> I3c1Rst {
        let val = (self.0 >> 1usize) & 0x01;
        I3c1Rst::from_bits(val as u8)
    }
    #[doc = "I3C1 reset control."]
    #[inline(always)]
    pub const fn set_i3c1_rst(&mut self, val: I3c1Rst) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SINC reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_rst(&self) -> SincRst {
        let val = (self.0 >> 2usize) & 0x01;
        SincRst::from_bits(val as u8)
    }
    #[doc = "SINC reset control."]
    #[inline(always)]
    pub const fn set_sinc_rst(&mut self, val: SincRst) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "CoolFlux reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_rst(&self) -> CoolfluxRst {
        let val = (self.0 >> 3usize) & 0x01;
        CoolfluxRst::from_bits(val as u8)
    }
    #[doc = "CoolFlux reset control."]
    #[inline(always)]
    pub const fn set_coolflux_rst(&mut self, val: CoolfluxRst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "QDC0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc0_rst(&self) -> Qdc0Rst {
        let val = (self.0 >> 4usize) & 0x01;
        Qdc0Rst::from_bits(val as u8)
    }
    #[doc = "QDC0 reset control."]
    #[inline(always)]
    pub const fn set_qdc0_rst(&mut self, val: Qdc0Rst) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "QDC1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn qdc1_rst(&self) -> Qdc1Rst {
        let val = (self.0 >> 5usize) & 0x01;
        Qdc1Rst::from_bits(val as u8)
    }
    #[doc = "QDC1 reset control."]
    #[inline(always)]
    pub const fn set_qdc1_rst(&mut self, val: Qdc1Rst) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "PWM0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm0_rst(&self) -> Pwm0Rst {
        let val = (self.0 >> 6usize) & 0x01;
        Pwm0Rst::from_bits(val as u8)
    }
    #[doc = "PWM0 reset control."]
    #[inline(always)]
    pub const fn set_pwm0_rst(&mut self, val: Pwm0Rst) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "PWM1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn pwm1_rst(&self) -> Pwm1Rst {
        let val = (self.0 >> 7usize) & 0x01;
        Pwm1Rst::from_bits(val as u8)
    }
    #[doc = "PWM1 reset control."]
    #[inline(always)]
    pub const fn set_pwm1_rst(&mut self, val: Pwm1Rst) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "AOI0 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn aoi0_rst(&self) -> Aoi0Rst {
        let val = (self.0 >> 8usize) & 0x01;
        Aoi0Rst::from_bits(val as u8)
    }
    #[doc = "AOI0 reset control."]
    #[inline(always)]
    pub const fn set_aoi0_rst(&mut self, val: Aoi0Rst) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DAC1 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dac1_rst(&self) -> Dac1Rst {
        let val = (self.0 >> 11usize) & 0x01;
        Dac1Rst::from_bits(val as u8)
    }
    #[doc = "DAC1 reset control."]
    #[inline(always)]
    pub const fn set_dac1_rst(&mut self, val: Dac1Rst) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DAC2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn dac2_rst(&self) -> Dac2Rst {
        let val = (self.0 >> 12usize) & 0x01;
        Dac2Rst::from_bits(val as u8)
    }
    #[doc = "DAC2 reset control."]
    #[inline(always)]
    pub const fn set_dac2_rst(&mut self, val: Dac2Rst) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "OPAMP reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn opamp_rst(&self, n: usize) -> OpampRst {
        assert!(n < 3usize);
        let offs = 13usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        OpampRst::from_bits(val as u8)
    }
    #[doc = "OPAMP reset control."]
    #[inline(always)]
    pub const fn set_opamp_rst(&mut self, n: usize, val: OpampRst) {
        assert!(n < 3usize);
        let offs = 13usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
    }
    #[doc = "CMP2 reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp2_rst(&self) -> Cmp2Rst {
        let val = (self.0 >> 18usize) & 0x01;
        Cmp2Rst::from_bits(val as u8)
    }
    #[doc = "CMP2 reset control."]
    #[inline(always)]
    pub const fn set_cmp2_rst(&mut self, val: Cmp2Rst) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "VREF reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn vref_rst(&self) -> VrefRst {
        let val = (self.0 >> 19usize) & 0x01;
        VrefRst::from_bits(val as u8)
    }
    #[doc = "VREF reset control."]
    #[inline(always)]
    pub const fn set_vref_rst(&mut self, val: VrefRst) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "CoolFlux APB reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn coolflux_apb_rst(&self) -> CoolfluxApbRst {
        let val = (self.0 >> 20usize) & 0x01;
        CoolfluxApbRst::from_bits(val as u8)
    }
    #[doc = "CoolFlux APB reset control."]
    #[inline(always)]
    pub const fn set_coolflux_apb_rst(&mut self, val: CoolfluxApbRst) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "NPU reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn npu_rst(&self) -> NpuRst {
        let val = (self.0 >> 21usize) & 0x01;
        NpuRst::from_bits(val as u8)
    }
    #[doc = "NPU reset control."]
    #[inline(always)]
    pub const fn set_npu_rst(&mut self, val: NpuRst) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "TSI reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn tsi_rst(&self) -> TsiRst {
        let val = (self.0 >> 22usize) & 0x01;
        TsiRst::from_bits(val as u8)
    }
    #[doc = "TSI reset control."]
    #[inline(always)]
    pub const fn set_tsi_rst(&mut self, val: TsiRst) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "EWM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn ewm_rst(&self) -> EwmRst {
        let val = (self.0 >> 23usize) & 0x01;
        EwmRst::from_bits(val as u8)
    }
    #[doc = "EWM reset control."]
    #[inline(always)]
    pub const fn set_ewm_rst(&mut self, val: EwmRst) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "EIM reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn eim_rst(&self) -> EimRst {
        let val = (self.0 >> 24usize) & 0x01;
        EimRst::from_bits(val as u8)
    }
    #[doc = "EIM reset control."]
    #[inline(always)]
    pub const fn set_eim_rst(&mut self, val: EimRst) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Semaphore reset control."]
    #[must_use]
    #[inline(always)]
    pub const fn sema42_rst(&self) -> Sema42Rst {
        let val = (self.0 >> 27usize) & 0x01;
        Sema42Rst::from_bits(val as u8)
    }
    #[doc = "Semaphore reset control."]
    #[inline(always)]
    pub const fn set_sema42_rst(&mut self, val: Sema42Rst) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Presetctrl3 {
    #[inline(always)]
    fn default() -> Presetctrl3 {
        Presetctrl3(0)
    }
}
impl core::fmt::Debug for Presetctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrl3")
            .field("i3c0_rst", &self.i3c0_rst())
            .field("i3c1_rst", &self.i3c1_rst())
            .field("sinc_rst", &self.sinc_rst())
            .field("coolflux_rst", &self.coolflux_rst())
            .field("qdc0_rst", &self.qdc0_rst())
            .field("qdc1_rst", &self.qdc1_rst())
            .field("pwm0_rst", &self.pwm0_rst())
            .field("pwm1_rst", &self.pwm1_rst())
            .field("aoi0_rst", &self.aoi0_rst())
            .field("dac1_rst", &self.dac1_rst())
            .field("dac2_rst", &self.dac2_rst())
            .field("opamp_rst[0]", &self.opamp_rst(0usize))
            .field("opamp_rst[1]", &self.opamp_rst(1usize))
            .field("opamp_rst[2]", &self.opamp_rst(2usize))
            .field("cmp2_rst", &self.cmp2_rst())
            .field("vref_rst", &self.vref_rst())
            .field("coolflux_apb_rst", &self.coolflux_apb_rst())
            .field("npu_rst", &self.npu_rst())
            .field("tsi_rst", &self.tsi_rst())
            .field("ewm_rst", &self.ewm_rst())
            .field("eim_rst", &self.eim_rst())
            .field("sema42_rst", &self.sema42_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Presetctrl3 {{ i3c0_rst: {:?}, i3c1_rst: {:?}, sinc_rst: {:?}, coolflux_rst: {:?}, qdc0_rst: {:?}, qdc1_rst: {:?}, pwm0_rst: {:?}, pwm1_rst: {:?}, aoi0_rst: {:?}, dac1_rst: {:?}, dac2_rst: {:?}, opamp_rst[0]: {:?}, opamp_rst[1]: {:?}, opamp_rst[2]: {:?}, cmp2_rst: {:?}, vref_rst: {:?}, coolflux_apb_rst: {:?}, npu_rst: {:?}, tsi_rst: {:?}, ewm_rst: {:?}, eim_rst: {:?}, sema42_rst: {:?} }}",
            self.i3c0_rst(),
            self.i3c1_rst(),
            self.sinc_rst(),
            self.coolflux_rst(),
            self.qdc0_rst(),
            self.qdc1_rst(),
            self.pwm0_rst(),
            self.pwm1_rst(),
            self.aoi0_rst(),
            self.dac1_rst(),
            self.dac2_rst(),
            self.opamp_rst(0usize),
            self.opamp_rst(1usize),
            self.opamp_rst(2usize),
            self.cmp2_rst(),
            self.vref_rst(),
            self.coolflux_apb_rst(),
            self.npu_rst(),
            self.tsi_rst(),
            self.ewm_rst(),
            self.eim_rst(),
            self.sema42_rst()
        )
    }
}
#[doc = "Peripheral Reset Control Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrlclr(pub u32);
impl Presetctrlclr {
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Presetctrlclr {
    #[inline(always)]
    fn default() -> Presetctrlclr {
        Presetctrlclr(0)
    }
}
impl core::fmt::Debug for Presetctrlclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrlclr")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrlclr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Presetctrlclr {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Peripheral Reset Control Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Presetctrlset(pub u32);
impl Presetctrlset {
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Data array value, refer to corresponding position in PRESETCTRLn."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Presetctrlset {
    #[inline(always)]
    fn default() -> Presetctrlset {
        Presetctrlset(0)
    }
}
impl core::fmt::Debug for Presetctrlset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Presetctrlset")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Presetctrlset {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Presetctrlset {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "PWM0 Submodule Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm0subctl(pub u32);
impl Pwm0subctl {
    #[doc = "Enables PWM0 SUB Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_en(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM0 SUB Clock."]
    #[inline(always)]
    pub const fn set_clk_en(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "PWM0 submodule DMA compare value done mask."]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 12usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "PWM0 submodule DMA compare value done mask."]
    #[inline(always)]
    pub const fn set_dmavalm(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 12usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pwm0subctl {
    #[inline(always)]
    fn default() -> Pwm0subctl {
        Pwm0subctl(0)
    }
}
impl core::fmt::Debug for Pwm0subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm0subctl")
            .field("clk_en[0]", &self.clk_en(0usize))
            .field("clk_en[1]", &self.clk_en(1usize))
            .field("clk_en[2]", &self.clk_en(2usize))
            .field("clk_en[3]", &self.clk_en(3usize))
            .field("dmavalm[0]", &self.dmavalm(0usize))
            .field("dmavalm[1]", &self.dmavalm(1usize))
            .field("dmavalm[2]", &self.dmavalm(2usize))
            .field("dmavalm[3]", &self.dmavalm(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm0subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm0subctl {{ clk_en[0]: {=bool:?}, clk_en[1]: {=bool:?}, clk_en[2]: {=bool:?}, clk_en[3]: {=bool:?}, dmavalm[0]: {=bool:?}, dmavalm[1]: {=bool:?}, dmavalm[2]: {=bool:?}, dmavalm[3]: {=bool:?} }}",
            self.clk_en(0usize),
            self.clk_en(1usize),
            self.clk_en(2usize),
            self.clk_en(3usize),
            self.dmavalm(0usize),
            self.dmavalm(1usize),
            self.dmavalm(2usize),
            self.dmavalm(3usize)
        )
    }
}
#[doc = "PWM1 Submodule Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm1subctl(pub u32);
impl Pwm1subctl {
    #[doc = "Enables PWM1 SUB Clock."]
    #[must_use]
    #[inline(always)]
    pub const fn clk_en(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Enables PWM1 SUB Clock."]
    #[inline(always)]
    pub const fn set_clk_en(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "PWM1 submodule DMA compare value done mask."]
    #[must_use]
    #[inline(always)]
    pub const fn dmavalm(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 12usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "PWM1 submodule DMA compare value done mask."]
    #[inline(always)]
    pub const fn set_dmavalm(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 12usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Pwm1subctl {
    #[inline(always)]
    fn default() -> Pwm1subctl {
        Pwm1subctl(0)
    }
}
impl core::fmt::Debug for Pwm1subctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwm1subctl")
            .field("clk_en[0]", &self.clk_en(0usize))
            .field("clk_en[1]", &self.clk_en(1usize))
            .field("clk_en[2]", &self.clk_en(2usize))
            .field("clk_en[3]", &self.clk_en(3usize))
            .field("dmavalm[0]", &self.dmavalm(0usize))
            .field("dmavalm[1]", &self.dmavalm(1usize))
            .field("dmavalm[2]", &self.dmavalm(2usize))
            .field("dmavalm[3]", &self.dmavalm(3usize))
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwm1subctl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwm1subctl {{ clk_en[0]: {=bool:?}, clk_en[1]: {=bool:?}, clk_en[2]: {=bool:?}, clk_en[3]: {=bool:?}, dmavalm[0]: {=bool:?}, dmavalm[1]: {=bool:?}, dmavalm[2]: {=bool:?}, dmavalm[3]: {=bool:?} }}",
            self.clk_en(0usize),
            self.clk_en(1usize),
            self.clk_en(2usize),
            self.clk_en(3usize),
            self.dmavalm(0usize),
            self.dmavalm(1usize),
            self.dmavalm(2usize),
            self.dmavalm(3usize)
        )
    }
}
#[doc = "Control PKC RAM Interleave Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RamInterleave(pub u32);
impl RamInterleave {
    #[doc = "Controls PKC RAM access for PKC RAM 0 and PKC RAM 1."]
    #[must_use]
    #[inline(always)]
    pub const fn interleave(&self) -> Interleave {
        let val = (self.0 >> 0usize) & 0x01;
        Interleave::from_bits(val as u8)
    }
    #[doc = "Controls PKC RAM access for PKC RAM 0 and PKC RAM 1."]
    #[inline(always)]
    pub const fn set_interleave(&mut self, val: Interleave) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for RamInterleave {
    #[inline(always)]
    fn default() -> RamInterleave {
        RamInterleave(0)
    }
}
impl core::fmt::Debug for RamInterleave {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RamInterleave")
            .field("interleave", &self.interleave())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RamInterleave {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RamInterleave {{ interleave: {:?} }}", self.interleave())
    }
}
#[doc = "FRO 48MHz Reference Clock Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefClkCtrl(pub u32);
impl RefClkCtrl {
    #[doc = "GDET reference clock enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_refclk_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET reference clock enable bit."]
    #[inline(always)]
    pub const fn set_gdet_refclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ELS TRNG reference clock enable bit."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_refclk_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ELS TRNG reference clock enable bit."]
    #[inline(always)]
    pub const fn set_trng_refclk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RefClkCtrl {
    #[inline(always)]
    fn default() -> RefClkCtrl {
        RefClkCtrl(0)
    }
}
impl core::fmt::Debug for RefClkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefClkCtrl")
            .field("gdet_refclk_en", &self.gdet_refclk_en())
            .field("trng_refclk_en", &self.trng_refclk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefClkCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RefClkCtrl {{ gdet_refclk_en: {=bool:?}, trng_refclk_en: {=bool:?} }}",
            self.gdet_refclk_en(),
            self.trng_refclk_en()
        )
    }
}
#[doc = "FRO 48MHz Reference Clock Control Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefClkCtrlClr(pub u32);
impl RefClkCtrlClr {
    #[doc = "GDET reference clock enable clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_refclk_en_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET reference clock enable clear bit."]
    #[inline(always)]
    pub const fn set_gdet_refclk_en_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ELS TRNG reference clock enable clear bit."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_refclk_en_clr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ELS TRNG reference clock enable clear bit."]
    #[inline(always)]
    pub const fn set_trng_refclk_en_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RefClkCtrlClr {
    #[inline(always)]
    fn default() -> RefClkCtrlClr {
        RefClkCtrlClr(0)
    }
}
impl core::fmt::Debug for RefClkCtrlClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefClkCtrlClr")
            .field("gdet_refclk_en_clr", &self.gdet_refclk_en_clr())
            .field("trng_refclk_en_clr", &self.trng_refclk_en_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefClkCtrlClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RefClkCtrlClr {{ gdet_refclk_en_clr: {=bool:?}, trng_refclk_en_clr: {=bool:?} }}",
            self.gdet_refclk_en_clr(),
            self.trng_refclk_en_clr()
        )
    }
}
#[doc = "FRO 48MHz Reference Clock Control Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefClkCtrlSet(pub u32);
impl RefClkCtrlSet {
    #[doc = "GDET reference clock enable set bit."]
    #[must_use]
    #[inline(always)]
    pub const fn gdet_refclk_en_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "GDET reference clock enable set bit."]
    #[inline(always)]
    pub const fn set_gdet_refclk_en_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ELS TRNG reference clock enable set bit."]
    #[must_use]
    #[inline(always)]
    pub const fn trng_refclk_en_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ELS TRNG reference clock enable set bit."]
    #[inline(always)]
    pub const fn set_trng_refclk_en_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RefClkCtrlSet {
    #[inline(always)]
    fn default() -> RefClkCtrlSet {
        RefClkCtrlSet(0)
    }
}
impl core::fmt::Debug for RefClkCtrlSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefClkCtrlSet")
            .field("gdet_refclk_en_set", &self.gdet_refclk_en_set())
            .field("trng_refclk_en_set", &self.trng_refclk_en_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefClkCtrlSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RefClkCtrlSet {{ gdet_refclk_en_set: {=bool:?}, trng_refclk_en_set: {=bool:?} }}",
            self.gdet_refclk_en_set(),
            self.trng_refclk_en_set()
        )
    }
}
#[doc = "ROM Wait State."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Romcr(pub u32);
impl Romcr {
    #[doc = "ROM waiting Arm core and other masters for one cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn rom_wait(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ROM waiting Arm core and other masters for one cycle."]
    #[inline(always)]
    pub const fn set_rom_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Romcr {
    #[inline(always)]
    fn default() -> Romcr {
        Romcr(0)
    }
}
impl core::fmt::Debug for Romcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Romcr")
            .field("rom_wait", &self.rom_wait())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Romcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Romcr {{ rom_wait: {=bool:?} }}", self.rom_wait())
    }
}
#[doc = "SAI0 Function Clock Division."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai0clkdiv(pub u32);
impl Sai0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Sai0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Sai0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Sai0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Sai0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Sai0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Sai0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Sai0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Sai0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Sai0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sai0clkdiv {
    #[inline(always)]
    fn default() -> Sai0clkdiv {
        Sai0clkdiv(0)
    }
}
impl core::fmt::Debug for Sai0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SAI0 Function Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai0clksel(pub u32);
impl Sai0clksel {
    #[doc = "Selects the clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Sai0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Sai0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Sai0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sai0clksel {
    #[inline(always)]
    fn default() -> Sai0clksel {
        Sai0clksel(0)
    }
}
impl core::fmt::Debug for Sai0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SAI1 Function Clock Division."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1clkdiv(pub u32);
impl Sai1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Sai1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Sai1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Sai1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Sai1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Sai1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Sai1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Sai1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Sai1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Sai1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sai1clkdiv {
    #[inline(always)]
    fn default() -> Sai1clkdiv {
        Sai1clkdiv(0)
    }
}
impl core::fmt::Debug for Sai1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sai1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SAI1 Function Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1clksel(pub u32);
impl Sai1clksel {
    #[doc = "Selects the clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Sai1clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Sai1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Sai1clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sai1clksel {
    #[inline(always)]
    fn default() -> Sai1clksel {
        Sai1clksel(0)
    }
}
impl core::fmt::Debug for Sai1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sai1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SCT/PWM Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctclkdiv(pub u32);
impl Sctclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> SctclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        SctclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: SctclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> SctclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        SctclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: SctclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> SctclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        SctclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: SctclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sctclkdiv {
    #[inline(always)]
    fn default() -> Sctclkdiv {
        Sctclkdiv(0)
    }
}
impl core::fmt::Debug for Sctclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Sctclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SCTimer/PWM Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctclksel(pub u32);
impl Sctclksel {
    #[doc = "Selects the SCTimer/PWM clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> SctclkselSel {
        let val = (self.0 >> 0usize) & 0x0f;
        SctclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the SCTimer/PWM clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: SctclkselSel) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
}
impl Default for Sctclksel {
    #[inline(always)]
    fn default() -> Sctclksel {
        Sctclksel(0)
    }
}
impl core::fmt::Debug for Sctclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sctclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sctclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sctclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SINC FILTER Function Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sincfiltclksel(pub u32);
impl Sincfiltclksel {
    #[doc = "Selects the SINC FILTER function clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> SincfiltclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        SincfiltclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the SINC FILTER function clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: SincfiltclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sincfiltclksel {
    #[inline(always)]
    fn default() -> Sincfiltclksel {
        Sincfiltclksel(0)
    }
}
impl core::fmt::Debug for Sincfiltclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sincfiltclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sincfiltclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Sincfiltclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "SLOW_CLK Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slowclkdiv(pub u32);
impl Slowclkdiv {
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> SlowclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        SlowclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: SlowclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> SlowclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        SlowclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: SlowclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> SlowclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        SlowclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: SlowclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Slowclkdiv {
    #[inline(always)]
    fn default() -> Slowclkdiv {
        Slowclkdiv(0)
    }
}
impl core::fmt::Debug for Slowclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slowclkdiv")
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slowclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slowclkdiv {{ reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "SmartDMA Interrupt Hijack."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmartDmaint(pub u32);
impl SmartDmaint {
    #[doc = "SmartDMA hijack NVIC IRQ1."]
    #[must_use]
    #[inline(always)]
    pub const fn int0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ1."]
    #[inline(always)]
    pub const fn set_int0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ17."]
    #[must_use]
    #[inline(always)]
    pub const fn int1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ17."]
    #[inline(always)]
    pub const fn set_int1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ18."]
    #[must_use]
    #[inline(always)]
    pub const fn int2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ18."]
    #[inline(always)]
    pub const fn set_int2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ29."]
    #[must_use]
    #[inline(always)]
    pub const fn int3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ29."]
    #[inline(always)]
    pub const fn set_int3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ30."]
    #[must_use]
    #[inline(always)]
    pub const fn int4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ30."]
    #[inline(always)]
    pub const fn set_int4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ31."]
    #[must_use]
    #[inline(always)]
    pub const fn int5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ31."]
    #[inline(always)]
    pub const fn set_int5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ32."]
    #[must_use]
    #[inline(always)]
    pub const fn int6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ32."]
    #[inline(always)]
    pub const fn set_int6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ33."]
    #[must_use]
    #[inline(always)]
    pub const fn int7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ33."]
    #[inline(always)]
    pub const fn set_int7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ34."]
    #[must_use]
    #[inline(always)]
    pub const fn int8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ34."]
    #[inline(always)]
    pub const fn set_int8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ35."]
    #[must_use]
    #[inline(always)]
    pub const fn int9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ35."]
    #[inline(always)]
    pub const fn set_int9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ36."]
    #[must_use]
    #[inline(always)]
    pub const fn int10(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ36."]
    #[inline(always)]
    pub const fn set_int10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ37."]
    #[must_use]
    #[inline(always)]
    pub const fn int11(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ37."]
    #[inline(always)]
    pub const fn set_int11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ38."]
    #[must_use]
    #[inline(always)]
    pub const fn int12(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ38."]
    #[inline(always)]
    pub const fn set_int12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ39."]
    #[must_use]
    #[inline(always)]
    pub const fn int13(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ39."]
    #[inline(always)]
    pub const fn set_int13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ40."]
    #[must_use]
    #[inline(always)]
    pub const fn int14(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ40."]
    #[inline(always)]
    pub const fn set_int14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ41."]
    #[must_use]
    #[inline(always)]
    pub const fn int15(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ41."]
    #[inline(always)]
    pub const fn set_int15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ42."]
    #[must_use]
    #[inline(always)]
    pub const fn int16(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ42."]
    #[inline(always)]
    pub const fn set_int16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ45."]
    #[must_use]
    #[inline(always)]
    pub const fn int17(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ45."]
    #[inline(always)]
    pub const fn set_int17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ47."]
    #[must_use]
    #[inline(always)]
    pub const fn int18(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ47."]
    #[inline(always)]
    pub const fn set_int18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ50."]
    #[must_use]
    #[inline(always)]
    pub const fn int19(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ50."]
    #[inline(always)]
    pub const fn set_int19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ51."]
    #[must_use]
    #[inline(always)]
    pub const fn int20(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ51."]
    #[inline(always)]
    pub const fn set_int20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ66."]
    #[must_use]
    #[inline(always)]
    pub const fn int21(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ66."]
    #[inline(always)]
    pub const fn set_int21(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ67."]
    #[must_use]
    #[inline(always)]
    pub const fn int22(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ67."]
    #[inline(always)]
    pub const fn set_int22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SmartDMA hijack NVIC IRQ77."]
    #[must_use]
    #[inline(always)]
    pub const fn int23(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SmartDMA hijack NVIC IRQ77."]
    #[inline(always)]
    pub const fn set_int23(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for SmartDmaint {
    #[inline(always)]
    fn default() -> SmartDmaint {
        SmartDmaint(0)
    }
}
impl core::fmt::Debug for SmartDmaint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmartDmaint")
            .field("int0", &self.int0())
            .field("int1", &self.int1())
            .field("int2", &self.int2())
            .field("int3", &self.int3())
            .field("int4", &self.int4())
            .field("int5", &self.int5())
            .field("int6", &self.int6())
            .field("int7", &self.int7())
            .field("int8", &self.int8())
            .field("int9", &self.int9())
            .field("int10", &self.int10())
            .field("int11", &self.int11())
            .field("int12", &self.int12())
            .field("int13", &self.int13())
            .field("int14", &self.int14())
            .field("int15", &self.int15())
            .field("int16", &self.int16())
            .field("int17", &self.int17())
            .field("int18", &self.int18())
            .field("int19", &self.int19())
            .field("int20", &self.int20())
            .field("int21", &self.int21())
            .field("int22", &self.int22())
            .field("int23", &self.int23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmartDmaint {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SmartDmaint {{ int0: {=bool:?}, int1: {=bool:?}, int2: {=bool:?}, int3: {=bool:?}, int4: {=bool:?}, int5: {=bool:?}, int6: {=bool:?}, int7: {=bool:?}, int8: {=bool:?}, int9: {=bool:?}, int10: {=bool:?}, int11: {=bool:?}, int12: {=bool:?}, int13: {=bool:?}, int14: {=bool:?}, int15: {=bool:?}, int16: {=bool:?}, int17: {=bool:?}, int18: {=bool:?}, int19: {=bool:?}, int20: {=bool:?}, int21: {=bool:?}, int22: {=bool:?}, int23: {=bool:?} }}",
            self.int0(),
            self.int1(),
            self.int2(),
            self.int3(),
            self.int4(),
            self.int5(),
            self.int6(),
            self.int7(),
            self.int8(),
            self.int9(),
            self.int10(),
            self.int11(),
            self.int12(),
            self.int13(),
            self.int14(),
            self.int15(),
            self.int16(),
            self.int17(),
            self.int18(),
            self.int19(),
            self.int20(),
            self.int21(),
            self.int22(),
            self.int23()
        )
    }
}
#[doc = "CPU0 Software Debug Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu0(pub u32);
impl SwdAccessCpu0 {
    #[doc = "CPU0 SWD-AP: 0x12345678."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> SwdAccessCpu0SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        SwdAccessCpu0SecCode::from_bits(val as u32)
    }
    #[doc = "CPU0 SWD-AP: 0x12345678."]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: SwdAccessCpu0SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu0 {
    #[inline(always)]
    fn default() -> SwdAccessCpu0 {
        SwdAccessCpu0(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu0")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu0 {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[doc = "CPU1 Software Debug Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessCpu1(pub u32);
impl SwdAccessCpu1 {
    #[doc = "Security code to allow CPU1 DAP: 0x12345678."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> SwdAccessCpu1SecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        SwdAccessCpu1SecCode::from_bits(val as u32)
    }
    #[doc = "Security code to allow CPU1 DAP: 0x12345678."]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: SwdAccessCpu1SecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessCpu1 {
    #[inline(always)]
    fn default() -> SwdAccessCpu1 {
        SwdAccessCpu1(0)
    }
}
impl core::fmt::Debug for SwdAccessCpu1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessCpu1")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessCpu1 {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[doc = "DSP Software Debug Access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwdAccessDsp(pub u32);
impl SwdAccessDsp {
    #[doc = "DSP SWD-AP: 0x12345678."]
    #[must_use]
    #[inline(always)]
    pub const fn sec_code(&self) -> SwdAccessDspSecCode {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        SwdAccessDspSecCode::from_bits(val as u32)
    }
    #[doc = "DSP SWD-AP: 0x12345678."]
    #[inline(always)]
    pub const fn set_sec_code(&mut self, val: SwdAccessDspSecCode) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for SwdAccessDsp {
    #[inline(always)]
    fn default() -> SwdAccessDsp {
        SwdAccessDsp(0)
    }
}
impl core::fmt::Debug for SwdAccessDsp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwdAccessDsp")
            .field("sec_code", &self.sec_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessDsp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SwdAccessDsp {{ sec_code: {:?} }}", self.sec_code())
    }
}
#[doc = "CPU0 System Tick Timer Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkdiv0(pub u32);
impl Systickclkdiv0 {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Systickclkdiv0Reset {
        let val = (self.0 >> 29usize) & 0x01;
        Systickclkdiv0Reset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Systickclkdiv0Reset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Systickclkdiv0Halt {
        let val = (self.0 >> 30usize) & 0x01;
        Systickclkdiv0Halt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Systickclkdiv0Halt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Systickclkdiv0Unstab {
        let val = (self.0 >> 31usize) & 0x01;
        Systickclkdiv0Unstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Systickclkdiv0Unstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Systickclkdiv0 {
    #[inline(always)]
    fn default() -> Systickclkdiv0 {
        Systickclkdiv0(0)
    }
}
impl core::fmt::Debug for Systickclkdiv0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclkdiv0")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkdiv0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickclkdiv0 {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CPU1 System Tick Timer Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclkdiv1(pub u32);
impl Systickclkdiv1 {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Systickclkdiv1Reset {
        let val = (self.0 >> 29usize) & 0x01;
        Systickclkdiv1Reset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Systickclkdiv1Reset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Systickclkdiv1Halt {
        let val = (self.0 >> 30usize) & 0x01;
        Systickclkdiv1Halt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Systickclkdiv1Halt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Systickclkdiv1Unstab {
        let val = (self.0 >> 31usize) & 0x01;
        Systickclkdiv1Unstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Systickclkdiv1Unstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Systickclkdiv1 {
    #[inline(always)]
    fn default() -> Systickclkdiv1 {
        Systickclkdiv1(0)
    }
}
impl core::fmt::Debug for Systickclkdiv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclkdiv1")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclkdiv1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Systickclkdiv1 {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "CPU0 System Tick Timer Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclksel0(pub u32);
impl Systickclksel0 {
    #[doc = "Selects the System Tick Timer for CPU0 source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Systickclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        Systickclksel0Sel::from_bits(val as u8)
    }
    #[doc = "Selects the System Tick Timer for CPU0 source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Systickclksel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Systickclksel0 {
    #[inline(always)]
    fn default() -> Systickclksel0 {
        Systickclksel0(0)
    }
}
impl core::fmt::Debug for Systickclksel0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclksel0")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclksel0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickclksel0 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "CPU1 System Tick Timer Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickclksel1(pub u32);
impl Systickclksel1 {
    #[doc = "Selects the System Tick Timer for CPU1 source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Systickclksel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        Systickclksel1Sel::from_bits(val as u8)
    }
    #[doc = "Selects the System Tick Timer for CPU1 source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Systickclksel1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Systickclksel1 {
    #[inline(always)]
    fn default() -> Systickclksel1 {
        Systickclksel1(0)
    }
}
impl core::fmt::Debug for Systickclksel1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Systickclksel1")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Systickclksel1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Systickclksel1 {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "TRACE Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclkdiv(pub u32);
impl Traceclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> TraceclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        TraceclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: TraceclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> TraceclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        TraceclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: TraceclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> TraceclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        TraceclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: TraceclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Traceclkdiv {
    #[inline(always)]
    fn default() -> Traceclkdiv {
        Traceclkdiv(0)
    }
}
impl core::fmt::Debug for Traceclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Traceclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Traceclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Traceclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "Trace Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Traceclksel(pub u32);
impl Traceclksel {
    #[doc = "Selects the trace clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> TraceclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        TraceclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the trace clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: TraceclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Traceclksel {
    #[inline(always)]
    fn default() -> Traceclksel {
        Traceclksel(0)
    }
}
impl core::fmt::Debug for Traceclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Traceclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Traceclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Traceclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "TSI Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsiclkdiv(pub u32);
impl Tsiclkdiv {
    #[doc = "Clock divider value:."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value:."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> TsiclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        TsiclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: TsiclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> TsiclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        TsiclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: TsiclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> TsiclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        TsiclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: TsiclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Tsiclkdiv {
    #[inline(always)]
    fn default() -> Tsiclkdiv {
        Tsiclkdiv(0)
    }
}
impl core::fmt::Debug for Tsiclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsiclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsiclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Tsiclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "TSI Function Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsiclksel(pub u32);
impl Tsiclksel {
    #[doc = "Selects the TSI function clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> TsiclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        TsiclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the TSI function clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: TsiclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Tsiclksel {
    #[inline(always)]
    fn default() -> Tsiclksel {
        Tsiclksel(0)
    }
}
impl core::fmt::Debug for Tsiclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsiclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsiclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tsiclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "uSDHC Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USdhcclkdiv(pub u32);
impl USdhcclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> USdhcclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        USdhcclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: USdhcclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> USdhcclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        USdhcclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: USdhcclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> USdhcclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        USdhcclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: USdhcclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for USdhcclkdiv {
    #[inline(always)]
    fn default() -> USdhcclkdiv {
        USdhcclkdiv(0)
    }
}
impl core::fmt::Debug for USdhcclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USdhcclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USdhcclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "USdhcclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "uSDHC Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct USdhcclksel(pub u32);
impl USdhcclksel {
    #[doc = "Selects the uSDHC clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> USdhcclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        USdhcclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the uSDHC clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: USdhcclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for USdhcclksel {
    #[inline(always)]
    fn default() -> USdhcclksel {
        USdhcclksel(0)
    }
}
impl core::fmt::Debug for USdhcclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USdhcclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for USdhcclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "USdhcclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "USB-FS Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0clkdiv(pub u32);
impl Usb0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Usb0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Usb0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Usb0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Usb0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Usb0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Usb0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Usb0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Usb0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Usb0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Usb0clkdiv {
    #[inline(always)]
    fn default() -> Usb0clkdiv {
        Usb0clkdiv(0)
    }
}
impl core::fmt::Debug for Usb0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Usb0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "USB-FS Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb0clksel(pub u32);
impl Usb0clksel {
    #[doc = "Selects the USB-FS clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Usb0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        Usb0clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the USB-FS clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Usb0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Usb0clksel {
    #[inline(always)]
    fn default() -> Usb0clksel {
        Usb0clksel(0)
    }
}
impl core::fmt::Debug for Usb0clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usb0clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usb0clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Usb0clksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "UTICK Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utickclkdiv(pub u32);
impl Utickclkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> UtickclkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        UtickclkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: UtickclkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> UtickclkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        UtickclkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: UtickclkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> UtickclkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        UtickclkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: UtickclkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Utickclkdiv {
    #[inline(always)]
    fn default() -> Utickclkdiv {
        Utickclkdiv(0)
    }
}
impl core::fmt::Debug for Utickclkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utickclkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utickclkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Utickclkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "UTICK Function Clock Source Select."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utickclksel(pub u32);
impl Utickclksel {
    #[doc = "Selects the clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> UtickclkselSel {
        let val = (self.0 >> 0usize) & 0x03;
        UtickclkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the clock source."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: UtickclkselSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Utickclksel {
    #[inline(always)]
    fn default() -> Utickclksel {
        Utickclksel(0)
    }
}
impl core::fmt::Debug for Utickclksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utickclksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utickclksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Utickclksel {{ sel: {:?} }}", self.sel())
    }
}
#[doc = "WDT0 Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt0clkdiv(pub u32);
impl Wdt0clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Wdt0clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Wdt0clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Wdt0clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Wdt0clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Wdt0clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Wdt0clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Wdt0clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Wdt0clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Wdt0clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Wdt0clkdiv {
    #[inline(always)]
    fn default() -> Wdt0clkdiv {
        Wdt0clkdiv(0)
    }
}
impl core::fmt::Debug for Wdt0clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt0clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt0clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdt0clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "WDT1 Function Clock Divider."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt1clkdiv(pub u32);
impl Wdt1clkdiv {
    #[doc = "Clock divider value."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Clock divider value."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Resets the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn reset(&self) -> Wdt1clkdivReset {
        let val = (self.0 >> 29usize) & 0x01;
        Wdt1clkdivReset::from_bits(val as u8)
    }
    #[doc = "Resets the divider counter."]
    #[inline(always)]
    pub const fn set_reset(&mut self, val: Wdt1clkdivReset) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter."]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> Wdt1clkdivHalt {
        let val = (self.0 >> 30usize) & 0x01;
        Wdt1clkdivHalt::from_bits(val as u8)
    }
    #[doc = "Halts the divider counter."]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: Wdt1clkdivHalt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag."]
    #[must_use]
    #[inline(always)]
    pub const fn unstab(&self) -> Wdt1clkdivUnstab {
        let val = (self.0 >> 31usize) & 0x01;
        Wdt1clkdivUnstab::from_bits(val as u8)
    }
    #[doc = "Divider status flag."]
    #[inline(always)]
    pub const fn set_unstab(&mut self, val: Wdt1clkdivUnstab) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Wdt1clkdiv {
    #[inline(always)]
    fn default() -> Wdt1clkdiv {
        Wdt1clkdiv(0)
    }
}
impl core::fmt::Debug for Wdt1clkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt1clkdiv")
            .field("div", &self.div())
            .field("reset", &self.reset())
            .field("halt", &self.halt())
            .field("unstab", &self.unstab())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt1clkdiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Wdt1clkdiv {{ div: {=u8:?}, reset: {:?}, halt: {:?}, unstab: {:?} }}",
            self.div(),
            self.reset(),
            self.halt(),
            self.unstab()
        )
    }
}
#[doc = "WDT1 Clock Selection."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt1clksel(pub u32);
impl Wdt1clksel {
    #[doc = "Selects the WDT1 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sel(&self) -> Wdt1clkselSel {
        let val = (self.0 >> 0usize) & 0x03;
        Wdt1clkselSel::from_bits(val as u8)
    }
    #[doc = "Selects the WDT1 clock."]
    #[inline(always)]
    pub const fn set_sel(&mut self, val: Wdt1clkselSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Wdt1clksel {
    #[inline(always)]
    fn default() -> Wdt1clksel {
        Wdt1clksel(0)
    }
}
impl core::fmt::Debug for Wdt1clksel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdt1clksel")
            .field("sel", &self.sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdt1clksel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wdt1clksel {{ sel: {:?} }}", self.sel())
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Adc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Adc0Rst {
        Adc0Rst::from_bits(val)
    }
}
impl From<Adc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Adc0Rst) -> u8 {
        Adc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Adc0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivHalt {
        Adc0clkdivHalt::from_bits(val)
    }
}
impl From<Adc0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivHalt) -> u8 {
        Adc0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Adc0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivReset {
        Adc0clkdivReset::from_bits(val)
    }
}
impl From<Adc0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivReset) -> u8 {
        Adc0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Adc0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkdivUnstab {
        Adc0clkdivUnstab::from_bits(val)
    }
}
impl From<Adc0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkdivUnstab) -> u8 {
        Adc0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc0clkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO 12 MHz clock."]
    Enum0x3 = 0x03,
    #[doc = "Clk_in."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Adc0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc0clkselSel {
        Adc0clkselSel::from_bits(val)
    }
}
impl From<Adc0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc0clkselSel) -> u8 {
        Adc0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Adc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Adc1Rst {
        Adc1Rst::from_bits(val)
    }
}
impl From<Adc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Adc1Rst) -> u8 {
        Adc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Adc1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivHalt {
        Adc1clkdivHalt::from_bits(val)
    }
}
impl From<Adc1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivHalt) -> u8 {
        Adc1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Adc1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivReset {
        Adc1clkdivReset::from_bits(val)
    }
}
impl From<Adc1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivReset) -> u8 {
        Adc1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Adc1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkdivUnstab {
        Adc1clkdivUnstab::from_bits(val)
    }
}
impl From<Adc1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkdivUnstab) -> u8 {
        Adc1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Adc1clkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO 12 MHz clock."]
    Enum0x3 = 0x03,
    #[doc = "Clk_in clock."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Adc1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Adc1clkselSel {
        Adc1clkselSel::from_bits(val)
    }
}
impl From<Adc1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Adc1clkselSel) -> u8 {
        Adc1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl AhbclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> AhbclkdivUnstab {
        AhbclkdivUnstab::from_bits(val)
    }
}
impl From<AhbclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: AhbclkdivUnstab) -> u8 {
        AhbclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma0 {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioDma0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma0 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma0 {
        AhbmatprioDma0::from_bits(val)
    }
}
impl From<AhbmatprioDma0> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma0) -> u8 {
        AhbmatprioDma0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbmatprioDma1 {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl AhbmatprioDma1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbmatprioDma1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbmatprioDma1 {
    #[inline(always)]
    fn from(val: u8) -> AhbmatprioDma1 {
        AhbmatprioDma1::from_bits(val)
    }
}
impl From<AhbmatprioDma1> for u8 {
    #[inline(always)]
    fn from(val: AhbmatprioDma1) -> u8 {
        AhbmatprioDma1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aoi0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Aoi0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aoi0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aoi0Rst {
    #[inline(always)]
    fn from(val: u8) -> Aoi0Rst {
        Aoi0Rst::from_bits(val)
    }
}
impl From<Aoi0Rst> for u8 {
    #[inline(always)]
    fn from(val: Aoi0Rst) -> u8 {
        Aoi0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AssetProtection {
    #[doc = "ELS asset is protected."]
    Value0 = 0x0,
    #[doc = "ELS asset is not protected."]
    Value1 = 0x01,
    #[doc = "ELS asset is protected."]
    Value2 = 0x02,
    #[doc = "ELS asset is protected."]
    Value3 = 0x03,
}
impl AssetProtection {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AssetProtection {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AssetProtection {
    #[inline(always)]
    fn from(val: u8) -> AssetProtection {
        AssetProtection::from_bits(val)
    }
}
impl From<AssetProtection> for u8 {
    #[inline(always)]
    fn from(val: AssetProtection) -> u8 {
        AssetProtection::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BootImage {
    #[doc = "Internal flash image 0."]
    Enum0 = 0x0,
    #[doc = "Internal flash image 1."]
    Enum1 = 0x01,
    #[doc = "FlexSPI flash image 0."]
    Enum2 = 0x02,
    #[doc = "FlexSPI flash image 1."]
    Enum3 = 0x03,
    #[doc = "Recovery SPI flash image."]
    Enum4 = 0x04,
    #[doc = "Serial boot image (write-memory and execute ISP command used)."]
    Enum5 = 0x05,
    #[doc = "Receive SB3 containing SB_JUMP command is used."]
    Enum6 = 0x06,
    #[doc = "Customer SBL/recovery image (Bank1 IFR0)."]
    Enum7 = 0x07,
    #[doc = "NXP MAD recovery image (Bank1 IFR0)."]
    Enum8 = 0x08,
    #[doc = "NXP ROM extension (NMPA - Bank0 IFR0)."]
    Enum9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl BootImage {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BootImage {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BootImage {
    #[inline(always)]
    fn from(val: u8) -> BootImage {
        BootImage::from_bits(val)
    }
}
impl From<BootImage> for u8 {
    #[inline(always)]
    fn from(val: BootImage) -> u8 {
        BootImage::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl ClkoutdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivHalt {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivHalt {
        ClkoutdivHalt::from_bits(val)
    }
}
impl From<ClkoutdivHalt> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivHalt) -> u8 {
        ClkoutdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl ClkoutdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivReset {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivReset {
        ClkoutdivReset::from_bits(val)
    }
}
impl From<ClkoutdivReset> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivReset) -> u8 {
        ClkoutdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl ClkoutdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> ClkoutdivUnstab {
        ClkoutdivUnstab::from_bits(val)
    }
}
impl From<ClkoutdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: ClkoutdivUnstab) -> u8 {
        ClkoutdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClkoutselSel {
    #[doc = "Main clock (main_clk)."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock (pll0_clk)."]
    Enum0x1 = 0x01,
    #[doc = "CLKIN clock (clk_in)."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF clock (fro_hf)."]
    Enum0x3 = 0x03,
    #[doc = "FRO 12 MHz clock (fro_12m)."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock (pll1_clk)."]
    Enum0x5 = 0x05,
    #[doc = "LP Oscillator clock (lp_osc)."]
    Enum0x6 = 0x06,
    #[doc = "USB PLL clock (usb_pll_clk)."]
    Enum0x7 = 0x07,
    #[doc = "No clock."]
    Enum0x8 = 0x08,
    #[doc = "No clock."]
    Enum0x9 = 0x09,
    #[doc = "No clock."]
    Enum0xA = 0x0a,
    #[doc = "No clock."]
    Enum0xB = 0x0b,
    #[doc = "No clock."]
    Enum0xC = 0x0c,
    #[doc = "No clock."]
    Enum0xD = 0x0d,
    #[doc = "No clock."]
    Enum0xE = 0x0e,
    #[doc = "No clock."]
    Enum0xF = 0x0f,
}
impl ClkoutselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClkoutselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClkoutselSel {
    #[inline(always)]
    fn from(val: u8) -> ClkoutselSel {
        ClkoutselSel::from_bits(val)
    }
}
impl From<ClkoutselSel> for u8 {
    #[inline(always)]
    fn from(val: ClkoutselSel) -> u8 {
        ClkoutselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrFlashCache {
    #[doc = "No clear flash cache."]
    Enable = 0x0,
    #[doc = "Clears flash cache."]
    Disable = 0x01,
}
impl ClrFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrFlashCache {
    #[inline(always)]
    fn from(val: u8) -> ClrFlashCache {
        ClrFlashCache::from_bits(val)
    }
}
impl From<ClrFlashCache> for u8 {
    #[inline(always)]
    fn from(val: ClrFlashCache) -> u8 {
        ClrFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ClrLpcac {
    #[doc = "Unclears the cache."]
    Enable = 0x0,
    #[doc = "Clears the cache."]
    Disable = 0x01,
}
impl ClrLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ClrLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ClrLpcac {
    #[inline(always)]
    fn from(val: u8) -> ClrLpcac {
        ClrLpcac::from_bits(val)
    }
}
impl From<ClrLpcac> for u8 {
    #[inline(always)]
    fn from(val: ClrLpcac) -> u8 {
        ClrLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp0rrclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp0rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp0rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp0rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp0rrclkselSel {
        Cmp0rrclkselSel::from_bits(val)
    }
}
impl From<Cmp0rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp0rrclkselSel) -> u8 {
        Cmp0rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp1rrclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp1rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp1rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp1rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp1rrclkselSel {
        Cmp1rrclkselSel::from_bits(val)
    }
}
impl From<Cmp1rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp1rrclkselSel) -> u8 {
        Cmp1rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Cmp2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2Rst {
    #[inline(always)]
    fn from(val: u8) -> Cmp2Rst {
        Cmp2Rst::from_bits(val)
    }
}
impl From<Cmp2Rst> for u8 {
    #[inline(always)]
    fn from(val: Cmp2Rst) -> u8 {
        Cmp2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp2rrclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock0."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Cmp2rrclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp2rrclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp2rrclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Cmp2rrclkselSel {
        Cmp2rrclkselSel::from_bits(val)
    }
}
impl From<Cmp2rrclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Cmp2rrclkselSel) -> u8 {
        Cmp2rrclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpfclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl CmpfclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpfclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpfclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> CmpfclkdivHalt {
        CmpfclkdivHalt::from_bits(val)
    }
}
impl From<CmpfclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: CmpfclkdivHalt) -> u8 {
        CmpfclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpfclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl CmpfclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpfclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpfclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> CmpfclkdivReset {
        CmpfclkdivReset::from_bits(val)
    }
}
impl From<CmpfclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: CmpfclkdivReset) -> u8 {
        CmpfclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpfclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl CmpfclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpfclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpfclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> CmpfclkdivUnstab {
        CmpfclkdivUnstab::from_bits(val)
    }
}
impl From<CmpfclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: CmpfclkdivUnstab) -> u8 {
        CmpfclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmpfclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "FRO_HF clock."]
    Enum2 = 0x02,
    #[doc = "FRO_12M clock."]
    Enum3 = 0x03,
    #[doc = "CLKIN clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl CmpfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmpfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmpfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> CmpfclkselSel {
        CmpfclkselSel::from_bits(val)
    }
}
impl From<CmpfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: CmpfclkselSel) -> u8 {
        CmpfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmprrclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl CmprrclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmprrclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmprrclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> CmprrclkdivHalt {
        CmprrclkdivHalt::from_bits(val)
    }
}
impl From<CmprrclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: CmprrclkdivHalt) -> u8 {
        CmprrclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmprrclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl CmprrclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmprrclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmprrclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> CmprrclkdivReset {
        CmprrclkdivReset::from_bits(val)
    }
}
impl From<CmprrclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: CmprrclkdivReset) -> u8 {
        CmprrclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CmprrclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl CmprrclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CmprrclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CmprrclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> CmprrclkdivUnstab {
        CmprrclkdivUnstab::from_bits(val)
    }
}
impl From<CmprrclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: CmprrclkdivUnstab) -> u8 {
        CmprrclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoolfluxApbRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl CoolfluxApbRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoolfluxApbRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoolfluxApbRst {
    #[inline(always)]
    fn from(val: u8) -> CoolfluxApbRst {
        CoolfluxApbRst::from_bits(val)
    }
}
impl From<CoolfluxApbRst> for u8 {
    #[inline(always)]
    fn from(val: CoolfluxApbRst) -> u8 {
        CoolfluxApbRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoolfluxRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl CoolfluxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CoolfluxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CoolfluxRst {
    #[inline(always)]
    fn from(val: u8) -> CoolfluxRst {
        CoolfluxRst::from_bits(val)
    }
}
impl From<CoolfluxRst> for u8 {
    #[inline(always)]
    fn from(val: CoolfluxRst) -> u8 {
        CoolfluxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0lockup {
    #[doc = "CPU is not in lockup."]
    Awake = 0x0,
    #[doc = "CPU is in lockup."]
    Sleeping = 0x01,
}
impl Cpu0lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu0lockup {
        Cpu0lockup::from_bits(val)
    }
}
impl From<Cpu0lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu0lockup) -> u8 {
        Cpu0lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalNoref {
    #[doc = "Reference clock is provided."]
    YesRef = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef = 0x01,
}
impl Cpu0nstckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalNoref {
        Cpu0nstckcalNoref::from_bits(val)
    }
}
impl From<Cpu0nstckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalNoref) -> u8 {
        Cpu0nstckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0nstckcalSkew {
    #[doc = "TENMS value is exact."]
    Exact = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact = 0x01,
}
impl Cpu0nstckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0nstckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0nstckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0nstckcalSkew {
        Cpu0nstckcalSkew::from_bits(val)
    }
}
impl From<Cpu0nstckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0nstckcalSkew) -> u8 {
        Cpu0nstckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0sleeping {
    #[doc = "CPU is not sleeping."]
    Awake = 0x0,
    #[doc = "CPU is sleeping."]
    Sleeping = 0x01,
}
impl Cpu0sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu0sleeping {
        Cpu0sleeping::from_bits(val)
    }
}
impl From<Cpu0sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu0sleeping) -> u8 {
        Cpu0sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalNoref {
    #[doc = "Reference clock is provided."]
    YesRef = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef = 0x01,
}
impl Cpu0stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalNoref {
        Cpu0stckcalNoref::from_bits(val)
    }
}
impl From<Cpu0stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalNoref) -> u8 {
        Cpu0stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0stckcalSkew {
    #[doc = "TENMS value is exact."]
    Exact = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact = 0x01,
}
impl Cpu0stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu0stckcalSkew {
        Cpu0stckcalSkew::from_bits(val)
    }
}
impl From<Cpu0stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu0stckcalSkew) -> u8 {
        Cpu0stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1lockup {
    #[doc = "CPU is not in lockup."]
    Awake = 0x0,
    #[doc = "CPU is in lockup."]
    Sleeping = 0x01,
}
impl Cpu1lockup {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1lockup {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1lockup {
    #[inline(always)]
    fn from(val: u8) -> Cpu1lockup {
        Cpu1lockup::from_bits(val)
    }
}
impl From<Cpu1lockup> for u8 {
    #[inline(always)]
    fn from(val: Cpu1lockup) -> u8 {
        Cpu1lockup::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1rsten {
    #[doc = "The CPU1 is not reset."]
    Released = 0x0,
    #[doc = "The CPU1 is reset."]
    Asserted = 0x01,
}
impl Cpu1rsten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1rsten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1rsten {
    #[inline(always)]
    fn from(val: u8) -> Cpu1rsten {
        Cpu1rsten::from_bits(val)
    }
}
impl From<Cpu1rsten> for u8 {
    #[inline(always)]
    fn from(val: Cpu1rsten) -> u8 {
        Cpu1rsten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1sleeping {
    #[doc = "CPU is not sleeping."]
    Awake = 0x0,
    #[doc = "CPU is sleeping."]
    Sleeping = 0x01,
}
impl Cpu1sleeping {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1sleeping {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1sleeping {
    #[inline(always)]
    fn from(val: u8) -> Cpu1sleeping {
        Cpu1sleeping::from_bits(val)
    }
}
impl From<Cpu1sleeping> for u8 {
    #[inline(always)]
    fn from(val: Cpu1sleeping) -> u8 {
        Cpu1sleeping::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1stckcalNoref {
    #[doc = "Reference clock is provided."]
    YesRef1 = 0x0,
    #[doc = "No reference clock is provided."]
    NoRef1 = 0x01,
}
impl Cpu1stckcalNoref {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1stckcalNoref {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1stckcalNoref {
    #[inline(always)]
    fn from(val: u8) -> Cpu1stckcalNoref {
        Cpu1stckcalNoref::from_bits(val)
    }
}
impl From<Cpu1stckcalNoref> for u8 {
    #[inline(always)]
    fn from(val: Cpu1stckcalNoref) -> u8 {
        Cpu1stckcalNoref::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1stckcalSkew {
    #[doc = "TENMS value is exact."]
    Exact1 = 0x0,
    #[doc = "TENMS value is not exact or not given."]
    Inexact1 = 0x01,
}
impl Cpu1stckcalSkew {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1stckcalSkew {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1stckcalSkew {
    #[inline(always)]
    fn from(val: u8) -> Cpu1stckcalSkew {
        Cpu1stckcalSkew::from_bits(val)
    }
}
impl From<Cpu1stckcalSkew> for u8 {
    #[inline(always)]
    fn from(val: Cpu1stckcalSkew) -> u8 {
        Cpu1stckcalSkew::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CrcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl CrcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CrcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CrcRst {
    #[inline(always)]
    fn from(val: u8) -> CrcRst {
        CrcRst::from_bits(val)
    }
}
impl From<CrcRst> for u8 {
    #[inline(always)]
    fn from(val: CrcRst) -> u8 {
        CrcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivHalt {
    #[doc = "Divider clock is running."]
    Enable = 0x0,
    #[doc = "Divider clock has stopped."]
    Disable = 0x01,
}
impl CtimerclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivHalt {
        CtimerclkdivHalt::from_bits(val)
    }
}
impl From<CtimerclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivHalt) -> u8 {
        CtimerclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkdivUnstab {
    #[doc = "Stable divider clock."]
    Enable = 0x0,
    #[doc = "Unstable clock frequency."]
    Disable = 0x01,
}
impl CtimerclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkdivUnstab {
        CtimerclkdivUnstab::from_bits(val)
    }
}
impl From<CtimerclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkdivUnstab) -> u8 {
        CtimerclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CtimerclkselSel {
    #[doc = "FRO_1M clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "PLL1_clk0 clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum0x3 = 0x03,
    #[doc = "FRO 12MHz clock."]
    Enum0x4 = 0x04,
    #[doc = "SAI0 MCLK IN clock."]
    Enum0x5 = 0x05,
    #[doc = "LP Oscillator clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
    #[doc = "SAI1 MCLK IN clock."]
    Enum0x8 = 0x08,
    #[doc = "SAI0 TX_BCLK clock."]
    Enum0x9 = 0x09,
    #[doc = "SAI0 RX_BCLK clock."]
    Enum0xA = 0x0a,
    #[doc = "SAI1 TX_BCLK clock."]
    Enum0xB = 0x0b,
    #[doc = "SAI1 RX_BCLK clock."]
    Enum0xC = 0x0c,
    #[doc = "No clock."]
    Enum0xD = 0x0d,
    #[doc = "No clock."]
    Enum0xE = 0x0e,
    #[doc = "No clock."]
    Enum0xF = 0x0f,
}
impl CtimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CtimerclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CtimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> CtimerclkselSel {
        CtimerclkselSel::from_bits(val)
    }
}
impl From<CtimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: CtimerclkselSel) -> u8 {
        CtimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dac0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dac0Rst {
        Dac0Rst::from_bits(val)
    }
}
impl From<Dac0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dac0Rst) -> u8 {
        Dac0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dac1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dac1Rst {
        Dac1Rst::from_bits(val)
    }
}
impl From<Dac1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dac1Rst) -> u8 {
        Dac1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dac2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dac2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dac2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dac2Rst {
    #[inline(always)]
    fn from(val: u8) -> Dac2Rst {
        Dac2Rst::from_bits(val)
    }
}
impl From<Dac2Rst> for u8 {
    #[inline(always)]
    fn from(val: Dac2Rst) -> u8 {
        Dac2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl DacclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> DacclkdivHalt {
        DacclkdivHalt::from_bits(val)
    }
}
impl From<DacclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: DacclkdivHalt) -> u8 {
        DacclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl DacclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> DacclkdivReset {
        DacclkdivReset::from_bits(val)
    }
}
impl From<DacclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: DacclkdivReset) -> u8 {
        DacclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl DacclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> DacclkdivUnstab {
        DacclkdivUnstab::from_bits(val)
    }
}
impl From<DacclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: DacclkdivUnstab) -> u8 {
        DacclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DacclkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "Clk_in."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF."]
    Enum0x3 = 0x03,
    #[doc = "FRO_12M."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl DacclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DacclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DacclkselSel {
    #[inline(always)]
    fn from(val: u8) -> DacclkselSel {
        DacclkselSel::from_bits(val)
    }
}
impl From<DacclkselSel> for u8 {
    #[inline(always)]
    fn from(val: DacclkselSel) -> u8 {
        DacclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Dbgen {
        DebugFeaturesCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Dbgen) -> u8 {
        DebugFeaturesCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Niden {
        DebugFeaturesCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Niden) -> u8 {
        DebugFeaturesCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spiden {
        DebugFeaturesCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spiden) -> u8 {
        DebugFeaturesCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu0Spniden {
        DebugFeaturesCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu0Spniden) -> u8 {
        DebugFeaturesCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Dbgen {
        DebugFeaturesCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Dbgen) -> u8 {
        DebugFeaturesCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesCpu1Niden {
        DebugFeaturesCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesCpu1Niden) -> u8 {
        DebugFeaturesCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Dbgen {
        DebugFeaturesDpCpu0Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Dbgen) -> u8 {
        DebugFeaturesDpCpu0Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Niden {
        DebugFeaturesDpCpu0Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Niden) -> u8 {
        DebugFeaturesDpCpu0Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spiden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spiden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spiden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spiden {
        DebugFeaturesDpCpu0Spiden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spiden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spiden) -> u8 {
        DebugFeaturesDpCpu0Spiden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu0Spniden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu0Spniden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu0Spniden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu0Spniden {
        DebugFeaturesDpCpu0Spniden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu0Spniden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu0Spniden) -> u8 {
        DebugFeaturesDpCpu0Spniden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Dbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Dbgen {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Dbgen {
        DebugFeaturesDpCpu1Dbgen::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Dbgen> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Dbgen) -> u8 {
        DebugFeaturesDpCpu1Dbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugFeaturesDpCpu1Niden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugFeaturesDpCpu1Niden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugFeaturesDpCpu1Niden {
    #[inline(always)]
    fn from(val: u8) -> DebugFeaturesDpCpu1Niden {
        DebugFeaturesDpCpu1Niden::from_bits(val)
    }
}
impl From<DebugFeaturesDpCpu1Niden> for u8 {
    #[inline(always)]
    fn from(val: DebugFeaturesDpCpu1Niden) -> u8 {
        DebugFeaturesDpCpu1Niden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisDataSpec {
    #[doc = "Enables data speculation."]
    Enable = 0x0,
    #[doc = "Disables data speculation."]
    Disable = 0x01,
}
impl DisDataSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisDataSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisDataSpec {
    #[inline(always)]
    fn from(val: u8) -> DisDataSpec {
        DisDataSpec::from_bits(val)
    }
}
impl From<DisDataSpec> for u8 {
    #[inline(always)]
    fn from(val: DisDataSpec) -> u8 {
        DisDataSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashCache {
    #[doc = "Enables flash cache."]
    Enable = 0x0,
    #[doc = "Disables flash cache."]
    Disable = 0x01,
}
impl DisFlashCache {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashCache {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashCache {
    #[inline(always)]
    fn from(val: u8) -> DisFlashCache {
        DisFlashCache::from_bits(val)
    }
}
impl From<DisFlashCache> for u8 {
    #[inline(always)]
    fn from(val: DisFlashCache) -> u8 {
        DisFlashCache::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashData {
    #[doc = "Enables flash data cache when DIS_FLASH_CACHE=0."]
    Enable = 0x0,
    #[doc = "Disables flash data cache."]
    Disable = 0x01,
}
impl DisFlashData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashData {
    #[inline(always)]
    fn from(val: u8) -> DisFlashData {
        DisFlashData::from_bits(val)
    }
}
impl From<DisFlashData> for u8 {
    #[inline(always)]
    fn from(val: DisFlashData) -> u8 {
        DisFlashData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashInst {
    #[doc = "Enables flash instruction cache when DIS_FLASH_CACHE=0."]
    Enable = 0x0,
    #[doc = "Disables flash instruction cache."]
    Disable = 0x01,
}
impl DisFlashInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashInst {
    #[inline(always)]
    fn from(val: u8) -> DisFlashInst {
        DisFlashInst::from_bits(val)
    }
}
impl From<DisFlashInst> for u8 {
    #[inline(always)]
    fn from(val: DisFlashInst) -> u8 {
        DisFlashInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisFlashSpec {
    #[doc = "Enables flash speculation."]
    Enable = 0x0,
    #[doc = "Disables flash speculation."]
    Disable = 0x01,
}
impl DisFlashSpec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisFlashSpec {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisFlashSpec {
    #[inline(always)]
    fn from(val: u8) -> DisFlashSpec {
        DisFlashSpec::from_bits(val)
    }
}
impl From<DisFlashSpec> for u8 {
    #[inline(always)]
    fn from(val: DisFlashSpec) -> u8 {
        DisFlashSpec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisLpcac {
    #[doc = "Enabled."]
    Enable = 0x0,
    #[doc = "Disabled."]
    Disable = 0x01,
}
impl DisLpcac {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisLpcac {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisLpcac {
    #[inline(always)]
    fn from(val: u8) -> DisLpcac {
        DisLpcac::from_bits(val)
    }
}
impl From<DisLpcac> for u8 {
    #[inline(always)]
    fn from(val: DisLpcac) -> u8 {
        DisLpcac::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrData {
    #[doc = "Enables bus error on multi-bit ECC error for data."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for data."]
    Disable = 0x01,
}
impl DisMbeccErrData {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrData {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrData {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrData {
        DisMbeccErrData::from_bits(val)
    }
}
impl From<DisMbeccErrData> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrData) -> u8 {
        DisMbeccErrData::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisMbeccErrInst {
    #[doc = "Enables bus error on multi-bit ECC error for instruction."]
    Enable = 0x0,
    #[doc = "Disables bus error on multi-bit ECC error for instruction."]
    Disable = 0x01,
}
impl DisMbeccErrInst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisMbeccErrInst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisMbeccErrInst {
    #[inline(always)]
    fn from(val: u8) -> DisMbeccErrInst {
        DisMbeccErrInst::from_bits(val)
    }
}
impl From<DisMbeccErrInst> for u8 {
    #[inline(always)]
    fn from(val: DisMbeccErrInst) -> u8 {
        DisMbeccErrInst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dma0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma0Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma0Rst {
        Dma0Rst::from_bits(val)
    }
}
impl From<Dma0Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma0Rst) -> u8 {
        Dma0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dma1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Dma1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dma1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dma1Rst {
    #[inline(always)]
    fn from(val: u8) -> Dma1Rst {
        Dma1Rst::from_bits(val)
    }
}
impl From<Dma1Rst> for u8 {
    #[inline(always)]
    fn from(val: Dma1Rst) -> u8 {
        Dma1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspDbgden {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DspDbgden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspDbgden {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspDbgden {
    #[inline(always)]
    fn from(val: u8) -> DspDbgden {
        DspDbgden::from_bits(val)
    }
}
impl From<DspDbgden> for u8 {
    #[inline(always)]
    fn from(val: DspDbgden) -> u8 {
        DspDbgden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DspDbgen {
    _RESERVED_0 = 0x0,
    #[doc = "Disables debug."]
    Disable = 0x01,
    #[doc = "Enables debug."]
    Enable = 0x02,
    _RESERVED_3 = 0x03,
}
impl DspDbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DspDbgen {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DspDbgen {
    #[inline(always)]
    fn from(val: u8) -> DspDbgen {
        DspDbgen::from_bits(val)
    }
}
impl From<DspDbgen> for u8 {
    #[inline(always)]
    fn from(val: DspDbgen) -> u8 {
        DspDbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EimRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl EimRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EimRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EimRst {
    #[inline(always)]
    fn from(val: u8) -> EimRst {
        EimRst::from_bits(val)
    }
}
impl From<EimRst> for u8 {
    #[inline(always)]
    fn from(val: EimRst) -> u8 {
        EimRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Emvsim0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkdivHalt {
        Emvsim0clkdivHalt::from_bits(val)
    }
}
impl From<Emvsim0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkdivHalt) -> u8 {
        Emvsim0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Emvsim0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkdivReset {
        Emvsim0clkdivReset::from_bits(val)
    }
}
impl From<Emvsim0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkdivReset) -> u8 {
        Emvsim0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Emvsim0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkdivUnstab {
        Emvsim0clkdivUnstab::from_bits(val)
    }
}
impl From<Emvsim0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkdivUnstab) -> u8 {
        Emvsim0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim0clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "FRO_12M clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock0."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Emvsim0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Emvsim0clkselSel {
        Emvsim0clkselSel::from_bits(val)
    }
}
impl From<Emvsim0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Emvsim0clkselSel) -> u8 {
        Emvsim0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Emvsim1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkdivHalt {
        Emvsim1clkdivHalt::from_bits(val)
    }
}
impl From<Emvsim1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkdivHalt) -> u8 {
        Emvsim1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Emvsim1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkdivReset {
        Emvsim1clkdivReset::from_bits(val)
    }
}
impl From<Emvsim1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkdivReset) -> u8 {
        Emvsim1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Emvsim1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkdivUnstab {
        Emvsim1clkdivUnstab::from_bits(val)
    }
}
impl From<Emvsim1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkdivUnstab) -> u8 {
        Emvsim1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Emvsim1clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "FRO_12M clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock0."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Emvsim1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Emvsim1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Emvsim1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Emvsim1clkselSel {
        Emvsim1clkselSel::from_bits(val)
    }
}
impl From<Emvsim1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Emvsim1clkselSel) -> u8 {
        Emvsim1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl EnetRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetRst {
    #[inline(always)]
    fn from(val: u8) -> EnetRst {
        EnetRst::from_bits(val)
    }
}
impl From<EnetRst> for u8 {
    #[inline(always)]
    fn from(val: EnetRst) -> u8 {
        EnetRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl EnetptprefclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkdivHalt {
        EnetptprefclkdivHalt::from_bits(val)
    }
}
impl From<EnetptprefclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkdivHalt) -> u8 {
        EnetptprefclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl EnetptprefclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkdivReset {
        EnetptprefclkdivReset::from_bits(val)
    }
}
impl From<EnetptprefclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkdivReset) -> u8 {
        EnetptprefclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl EnetptprefclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkdivUnstab {
        EnetptprefclkdivUnstab::from_bits(val)
    }
}
impl From<EnetptprefclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkdivUnstab) -> u8 {
        EnetptprefclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetptprefclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "enet0_tx_clk clock."]
    Enum4 = 0x04,
    #[doc = "pll1_clk1 clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl EnetptprefclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetptprefclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetptprefclkselSel {
    #[inline(always)]
    fn from(val: u8) -> EnetptprefclkselSel {
        EnetptprefclkselSel::from_bits(val)
    }
}
impl From<EnetptprefclkselSel> for u8 {
    #[inline(always)]
    fn from(val: EnetptprefclkselSel) -> u8 {
        EnetptprefclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl EnetrmiiclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkdivHalt {
        EnetrmiiclkdivHalt::from_bits(val)
    }
}
impl From<EnetrmiiclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkdivHalt) -> u8 {
        EnetrmiiclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl EnetrmiiclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkdivReset {
        EnetrmiiclkdivReset::from_bits(val)
    }
}
impl From<EnetrmiiclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkdivReset) -> u8 {
        EnetrmiiclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl EnetrmiiclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkdivUnstab {
        EnetrmiiclkdivUnstab::from_bits(val)
    }
}
impl From<EnetrmiiclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkdivUnstab) -> u8 {
        EnetrmiiclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnetrmiiclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl EnetrmiiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnetrmiiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnetrmiiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> EnetrmiiclkselSel {
        EnetrmiiclkselSel::from_bits(val)
    }
}
impl From<EnetrmiiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: EnetrmiiclkselSel) -> u8 {
        EnetrmiiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evsim0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Evsim0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evsim0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evsim0Rst {
    #[inline(always)]
    fn from(val: u8) -> Evsim0Rst {
        Evsim0Rst::from_bits(val)
    }
}
impl From<Evsim0Rst> for u8 {
    #[inline(always)]
    fn from(val: Evsim0Rst) -> u8 {
        Evsim0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Evsim1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Evsim1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Evsim1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Evsim1Rst {
    #[inline(always)]
    fn from(val: u8) -> Evsim1Rst {
        Evsim1Rst::from_bits(val)
    }
}
impl From<Evsim1Rst> for u8 {
    #[inline(always)]
    fn from(val: Evsim1Rst) -> u8 {
        Evsim1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ewm0clkselSel {
    #[doc = "clk_16k\\[2\\]."]
    Enum0 = 0x0,
    #[doc = "xtal32k\\[2\\]."]
    Enum1 = 0x01,
}
impl Ewm0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ewm0clkselSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ewm0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Ewm0clkselSel {
        Ewm0clkselSel::from_bits(val)
    }
}
impl From<Ewm0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Ewm0clkselSel) -> u8 {
        Ewm0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EwmRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl EwmRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EwmRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EwmRst {
    #[inline(always)]
    fn from(val: u8) -> EwmRst {
        EwmRst::from_bits(val)
    }
}
impl From<EwmRst> for u8 {
    #[inline(always)]
    fn from(val: EwmRst) -> u8 {
        EwmRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcRst {
    #[inline(always)]
    fn from(val: u8) -> FcRst {
        FcRst::from_bits(val)
    }
}
impl From<FcRst> for u8 {
    #[inline(always)]
    fn from(val: FcRst) -> u8 {
        FcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FcclkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL divided clock."]
    Enum0x1 = 0x01,
    #[doc = "FRO 12 MHz clock."]
    Enum0x2 = 0x02,
    #[doc = "fro_hf_div clock."]
    Enum0x3 = 0x03,
    #[doc = "clk_1m clock."]
    Enum0x4 = 0x04,
    #[doc = "USB PLL clock."]
    Enum0x5 = 0x05,
    #[doc = "LP Oscillator clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl FcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FcclkselSel {
        FcclkselSel::from_bits(val)
    }
}
impl From<FcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FcclkselSel) -> u8 {
        FcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlashStallEn {
    #[doc = "No stall on FLASH busy."]
    Enable = 0x0,
    #[doc = "Stall on FLASH busy."]
    Disable = 0x01,
}
impl FlashStallEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlashStallEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlashStallEn {
    #[inline(always)]
    fn from(val: u8) -> FlashStallEn {
        FlashStallEn::from_bits(val)
    }
}
impl From<FlashStallEn> for u8 {
    #[inline(always)]
    fn from(val: FlashStallEn) -> u8 {
        FlashStallEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FlexSpiclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkdivHalt {
        FlexSpiclkdivHalt::from_bits(val)
    }
}
impl From<FlexSpiclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkdivHalt) -> u8 {
        FlexSpiclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FlexSpiclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkdivReset {
        FlexSpiclkdivReset::from_bits(val)
    }
}
impl From<FlexSpiclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkdivReset) -> u8 {
        FlexSpiclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FlexSpiclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkdivUnstab {
        FlexSpiclkdivUnstab::from_bits(val)
    }
}
impl From<FlexSpiclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkdivUnstab) -> u8 {
        FlexSpiclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexSpiclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "pll1_clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
    #[doc = "No clock."]
    Enum8 = 0x08,
    #[doc = "No clock."]
    Enum9 = 0x09,
    #[doc = "No clock."]
    Enum10 = 0x0a,
    #[doc = "No clock."]
    Enum11 = 0x0b,
    #[doc = "No clock."]
    Enum12 = 0x0c,
    #[doc = "No clock."]
    Enum13 = 0x0d,
    #[doc = "No clock."]
    Enum14 = 0x0e,
    #[doc = "No clock."]
    Enum15 = 0x0f,
}
impl FlexSpiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexSpiclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexSpiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexSpiclkselSel {
        FlexSpiclkselSel::from_bits(val)
    }
}
impl From<FlexSpiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexSpiclkselSel) -> u8 {
        FlexSpiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Flexcan0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0Rst {
        Flexcan0Rst::from_bits(val)
    }
}
impl From<Flexcan0Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0Rst) -> u8 {
        Flexcan0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Flexcan0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivHalt {
        Flexcan0clkdivHalt::from_bits(val)
    }
}
impl From<Flexcan0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivHalt) -> u8 {
        Flexcan0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Flexcan0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivReset {
        Flexcan0clkdivReset::from_bits(val)
    }
}
impl From<Flexcan0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivReset) -> u8 {
        Flexcan0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Flexcan0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkdivUnstab {
        Flexcan0clkdivUnstab::from_bits(val)
    }
}
impl From<Flexcan0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkdivUnstab) -> u8 {
        Flexcan0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan0clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Flexcan0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan0clkselSel {
        Flexcan0clkselSel::from_bits(val)
    }
}
impl From<Flexcan0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan0clkselSel) -> u8 {
        Flexcan0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Flexcan1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1Rst {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1Rst {
        Flexcan1Rst::from_bits(val)
    }
}
impl From<Flexcan1Rst> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1Rst) -> u8 {
        Flexcan1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Flexcan1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivHalt {
        Flexcan1clkdivHalt::from_bits(val)
    }
}
impl From<Flexcan1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivHalt) -> u8 {
        Flexcan1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Flexcan1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivReset {
        Flexcan1clkdivReset::from_bits(val)
    }
}
impl From<Flexcan1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivReset) -> u8 {
        Flexcan1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Flexcan1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkdivUnstab {
        Flexcan1clkdivUnstab::from_bits(val)
    }
}
impl From<Flexcan1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkdivUnstab) -> u8 {
        Flexcan1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Flexcan1clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Flexcan1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Flexcan1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Flexcan1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Flexcan1clkselSel {
        Flexcan1clkselSel::from_bits(val)
    }
}
impl From<Flexcan1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Flexcan1clkselSel) -> u8 {
        Flexcan1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FlexcommclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivHalt {
        FlexcommclkdivHalt::from_bits(val)
    }
}
impl From<FlexcommclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivHalt) -> u8 {
        FlexcommclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FlexcommclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivReset {
        FlexcommclkdivReset::from_bits(val)
    }
}
impl From<FlexcommclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivReset) -> u8 {
        FlexcommclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexcommclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FlexcommclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexcommclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexcommclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexcommclkdivUnstab {
        FlexcommclkdivUnstab::from_bits(val)
    }
}
impl From<FlexcommclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexcommclkdivUnstab) -> u8 {
        FlexcommclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FlexioRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioRst {
    #[inline(always)]
    fn from(val: u8) -> FlexioRst {
        FlexioRst::from_bits(val)
    }
}
impl From<FlexioRst> for u8 {
    #[inline(always)]
    fn from(val: FlexioRst) -> u8 {
        FlexioRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FlexioclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivHalt {
        FlexioclkdivHalt::from_bits(val)
    }
}
impl From<FlexioclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivHalt) -> u8 {
        FlexioclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl FlexioclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivReset {
        FlexioclkdivReset::from_bits(val)
    }
}
impl From<FlexioclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivReset) -> u8 {
        FlexioclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FlexioclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkdivUnstab {
        FlexioclkdivUnstab::from_bits(val)
    }
}
impl From<FlexioclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkdivUnstab) -> u8 {
        FlexioclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexioclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "FRO_12M clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl FlexioclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexioclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexioclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexioclkselSel {
        FlexioclkselSel::from_bits(val)
    }
}
impl From<FlexioclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexioclkselSel) -> u8 {
        FlexioclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FlexspiRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FlexspiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspiRst {
    #[inline(always)]
    fn from(val: u8) -> FlexspiRst {
        FlexspiRst::from_bits(val)
    }
}
impl From<FlexspiRst> for u8 {
    #[inline(always)]
    fn from(val: FlexspiRst) -> u8 {
        FlexspiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FmuRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FmuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FmuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FmuRst {
    #[inline(always)]
    fn from(val: u8) -> FmuRst {
        FmuRst::from_bits(val)
    }
}
impl From<FmuRst> for u8 {
    #[inline(always)]
    fn from(val: FmuRst) -> u8 {
        FmuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrcNoAlloc {
    #[doc = "Forces allocation."]
    Enable = 0x0,
    #[doc = "Forces no allocation."]
    Disable = 0x01,
}
impl FrcNoAlloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrcNoAlloc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrcNoAlloc {
    #[inline(always)]
    fn from(val: u8) -> FrcNoAlloc {
        FrcNoAlloc::from_bits(val)
    }
}
impl From<FrcNoAlloc> for u8 {
    #[inline(always)]
    fn from(val: FrcNoAlloc) -> u8 {
        FrcNoAlloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FreqmeRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl FreqmeRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FreqmeRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FreqmeRst {
    #[inline(always)]
    fn from(val: u8) -> FreqmeRst {
        FreqmeRst::from_bits(val)
    }
}
impl From<FreqmeRst> for u8 {
    #[inline(always)]
    fn from(val: FreqmeRst) -> u8 {
        FreqmeRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivHalt {
    #[doc = "Divider clock is running, this bit is set to 0 when the register is written."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl FrohfdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivHalt {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivHalt {
        FrohfdivHalt::from_bits(val)
    }
}
impl From<FrohfdivHalt> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivHalt) -> u8 {
        FrohfdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrohfdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl FrohfdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FrohfdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FrohfdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> FrohfdivUnstab {
        FrohfdivUnstab::from_bits(val)
    }
}
impl From<FrohfdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: FrohfdivUnstab) -> u8 {
        FrohfdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GdetIsoSw {
    #[doc = "Isolation is disabled."]
    Disable0 = 0x0,
    #[doc = "Isolation is disabled."]
    Disable1 = 0x01,
    #[doc = "Isolation is enabled. When both GDET0_CTRL/GDET1_CTRL GDET_ISO_SW are \"10\", isolation_on is asserted."]
    Enable = 0x02,
    #[doc = "Isolation is disabled."]
    Disable3 = 0x03,
}
impl GdetIsoSw {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GdetIsoSw {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GdetIsoSw {
    #[inline(always)]
    fn from(val: u8) -> GdetIsoSw {
        GdetIsoSw::from_bits(val)
    }
}
impl From<GdetIsoSw> for u8 {
    #[inline(always)]
    fn from(val: GdetIsoSw) -> u8 {
        GdetIsoSw::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpioRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl GpioRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpioRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpioRst {
    #[inline(always)]
    fn from(val: u8) -> GpioRst {
        GpioRst::from_bits(val)
    }
}
impl From<GpioRst> for u8 {
    #[inline(always)]
    fn from(val: GpioRst) -> u8 {
        GpioRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl I3c0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c0Rst {
        I3c0Rst::from_bits(val)
    }
}
impl From<I3c0Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c0Rst) -> u8 {
        I3c0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c0fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivHalt {
        I3c0fclkdivHalt::from_bits(val)
    }
}
impl From<I3c0fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivHalt) -> u8 {
        I3c0fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c0fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivReset {
        I3c0fclkdivReset::from_bits(val)
    }
}
impl From<I3c0fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivReset) -> u8 {
        I3c0fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c0fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkdivUnstab {
        I3c0fclkdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkdivUnstab) -> u8 {
        I3c0fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c0fclksdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksdivHalt {
        I3c0fclksdivHalt::from_bits(val)
    }
}
impl From<I3c0fclksdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksdivHalt) -> u8 {
        I3c0fclksdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c0fclksdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksdivReset {
        I3c0fclksdivReset::from_bits(val)
    }
}
impl From<I3c0fclksdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksdivReset) -> u8 {
        I3c0fclksdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c0fclksdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksdivUnstab {
        I3c0fclksdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclksdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksdivUnstab) -> u8 {
        I3c0fclksdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkselSel {
        I3c0fclkselSel::from_bits(val)
    }
}
impl From<I3c0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkselSel) -> u8 {
        I3c0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclksselSel {
    #[doc = "FRO_1M clock."]
    Enum0 = 0x0,
    #[doc = "No clock."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c0fclksselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclksselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclksselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclksselSel {
        I3c0fclksselSel::from_bits(val)
    }
}
impl From<I3c0fclksselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclksselSel) -> u8 {
        I3c0fclksselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c0fclkstcdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcdivHalt {
        I3c0fclkstcdivHalt::from_bits(val)
    }
}
impl From<I3c0fclkstcdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcdivHalt) -> u8 {
        I3c0fclkstcdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c0fclkstcdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcdivReset {
        I3c0fclkstcdivReset::from_bits(val)
    }
}
impl From<I3c0fclkstcdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcdivReset) -> u8 {
        I3c0fclkstcdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c0fclkstcdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcdivUnstab {
        I3c0fclkstcdivUnstab::from_bits(val)
    }
}
impl From<I3c0fclkstcdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcdivUnstab) -> u8 {
        I3c0fclkstcdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c0fclkstcselSel {
    #[doc = "I3C0 functional clock I3C0FCLK."]
    Enum0 = 0x0,
    #[doc = "FRO_1M clock."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c0fclkstcselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c0fclkstcselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c0fclkstcselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c0fclkstcselSel {
        I3c0fclkstcselSel::from_bits(val)
    }
}
impl From<I3c0fclkstcselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c0fclkstcselSel) -> u8 {
        I3c0fclkstcselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl I3c1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1Rst {
    #[inline(always)]
    fn from(val: u8) -> I3c1Rst {
        I3c1Rst::from_bits(val)
    }
}
impl From<I3c1Rst> for u8 {
    #[inline(always)]
    fn from(val: I3c1Rst) -> u8 {
        I3c1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c1fclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivHalt {
        I3c1fclkdivHalt::from_bits(val)
    }
}
impl From<I3c1fclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivHalt) -> u8 {
        I3c1fclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c1fclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivReset {
        I3c1fclkdivReset::from_bits(val)
    }
}
impl From<I3c1fclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivReset) -> u8 {
        I3c1fclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c1fclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkdivUnstab {
        I3c1fclkdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkdivUnstab) -> u8 {
        I3c1fclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c1fclksdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksdivHalt {
        I3c1fclksdivHalt::from_bits(val)
    }
}
impl From<I3c1fclksdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksdivHalt) -> u8 {
        I3c1fclksdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c1fclksdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksdivReset {
        I3c1fclksdivReset::from_bits(val)
    }
}
impl From<I3c1fclksdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksdivReset) -> u8 {
        I3c1fclksdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c1fclksdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksdivUnstab {
        I3c1fclksdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclksdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksdivUnstab) -> u8 {
        I3c1fclksdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkselSel {
        I3c1fclkselSel::from_bits(val)
    }
}
impl From<I3c1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkselSel) -> u8 {
        I3c1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclksselSel {
    #[doc = "FRO_1M clock."]
    Enum0 = 0x0,
    #[doc = "No clock."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c1fclksselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclksselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclksselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclksselSel {
        I3c1fclksselSel::from_bits(val)
    }
}
impl From<I3c1fclksselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclksselSel) -> u8 {
        I3c1fclksselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl I3c1fclkstcdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcdivHalt {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcdivHalt {
        I3c1fclkstcdivHalt::from_bits(val)
    }
}
impl From<I3c1fclkstcdivHalt> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcdivHalt) -> u8 {
        I3c1fclkstcdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl I3c1fclkstcdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcdivReset {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcdivReset {
        I3c1fclkstcdivReset::from_bits(val)
    }
}
impl From<I3c1fclkstcdivReset> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcdivReset) -> u8 {
        I3c1fclkstcdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl I3c1fclkstcdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcdivUnstab {
        I3c1fclkstcdivUnstab::from_bits(val)
    }
}
impl From<I3c1fclkstcdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcdivUnstab) -> u8 {
        I3c1fclkstcdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum I3c1fclkstcselSel {
    #[doc = "I3C1 functional clock I3C1FCLK."]
    Enum0 = 0x0,
    #[doc = "FRO_1M clock."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl I3c1fclkstcselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> I3c1fclkstcselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for I3c1fclkstcselSel {
    #[inline(always)]
    fn from(val: u8) -> I3c1fclkstcselSel {
        I3c1fclkstcselSel::from_bits(val)
    }
}
impl From<I3c1fclkstcselSel> for u8 {
    #[inline(always)]
    fn from(val: I3c1fclkstcselSel) -> u8 {
        I3c1fclkstcselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interleave {
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is consecutive."]
    Normal = 0x0,
    #[doc = "RAM access to PKC RAM 0 and PKC RAM 1 is interleaved. This setting is need for PKC L0 memory access."]
    Interleave = 0x01,
}
impl Interleave {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Interleave {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Interleave {
    #[inline(always)]
    fn from(val: u8) -> Interleave {
        Interleave::from_bits(val)
    }
}
impl From<Interleave> for u8 {
    #[inline(always)]
    fn from(val: Interleave) -> u8 {
        Interleave::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockAll {
    #[doc = "Any other value than b1010: disables write access to all registers."]
    Disable = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    #[doc = "Enables write access to all registers."]
    Enable = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl LockAll {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockAll {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockAll {
    #[inline(always)]
    fn from(val: u8) -> LockAll {
        LockAll::from_bits(val)
    }
}
impl From<LockAll> for u8 {
    #[inline(always)]
    fn from(val: LockAll) -> u8 {
        LockAll::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MailboxRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MailboxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MailboxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MailboxRst {
    #[inline(always)]
    fn from(val: u8) -> MailboxRst {
        MailboxRst::from_bits(val)
    }
}
impl From<MailboxRst> for u8 {
    #[inline(always)]
    fn from(val: MailboxRst) -> u8 {
        MailboxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MicfilRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilRst {
    #[inline(always)]
    fn from(val: u8) -> MicfilRst {
        MicfilRst::from_bits(val)
    }
}
impl From<MicfilRst> for u8 {
    #[inline(always)]
    fn from(val: MicfilRst) -> u8 {
        MicfilRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl MicfilfclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivHalt {
        MicfilfclkdivHalt::from_bits(val)
    }
}
impl From<MicfilfclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivHalt) -> u8 {
        MicfilfclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl MicfilfclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivReset {
        MicfilfclkdivReset::from_bits(val)
    }
}
impl From<MicfilfclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivReset) -> u8 {
        MicfilfclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl MicfilfclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkdivUnstab {
        MicfilfclkdivUnstab::from_bits(val)
    }
}
impl From<MicfilfclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkdivUnstab) -> u8 {
        MicfilfclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicfilfclkselSel {
    #[doc = "FRO_12M clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "PLL1_clk0 clock."]
    Enum4 = 0x04,
    #[doc = "SAI0_MCLK clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
    #[doc = "SAI1_MCLK clock."]
    Enum8 = 0x08,
    #[doc = "No clock."]
    Enum9 = 0x09,
    #[doc = "No clock."]
    Enum10 = 0x0a,
    #[doc = "No clock."]
    Enum11 = 0x0b,
    #[doc = "No clock."]
    Enum12 = 0x0c,
    #[doc = "No clock."]
    Enum13 = 0x0d,
    #[doc = "No clock."]
    Enum14 = 0x0e,
    #[doc = "No clock."]
    Enum15 = 0x0f,
}
impl MicfilfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MicfilfclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MicfilfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> MicfilfclkselSel {
        MicfilfclkselSel::from_bits(val)
    }
}
impl From<MicfilfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: MicfilfclkselSel) -> u8 {
        MicfilfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MrtRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MrtRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MrtRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MrtRst {
    #[inline(always)]
    fn from(val: u8) -> MrtRst {
        MrtRst::from_bits(val)
    }
}
impl From<MrtRst> for u8 {
    #[inline(always)]
    fn from(val: MrtRst) -> u8 {
        MrtRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl MuxRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRst {
    #[inline(always)]
    fn from(val: u8) -> MuxRst {
        MuxRst::from_bits(val)
    }
}
impl From<MuxRst> for u8 {
    #[inline(always)]
    fn from(val: MuxRst) -> u8 {
        MuxRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NpuRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl NpuRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NpuRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NpuRst {
    #[inline(always)]
    fn from(val: u8) -> NpuRst {
        NpuRst::from_bits(val)
    }
}
impl From<NpuRst> for u8 {
    #[inline(always)]
    fn from(val: NpuRst) -> u8 {
        NpuRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OpampRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl OpampRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OpampRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OpampRst {
    #[inline(always)]
    fn from(val: u8) -> OpampRst {
        OpampRst::from_bits(val)
    }
}
impl From<OpampRst> for u8 {
    #[inline(always)]
    fn from(val: OpampRst) -> u8 {
        OpampRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl OstimerRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerRst {
    #[inline(always)]
    fn from(val: u8) -> OstimerRst {
        OstimerRst::from_bits(val)
    }
}
impl From<OstimerRst> for u8 {
    #[inline(always)]
    fn from(val: OstimerRst) -> u8 {
        OstimerRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OstimerclkselSel {
    #[doc = "clk_16k\\[2\\]."]
    Enum0 = 0x0,
    #[doc = "xtal32k\\[2\\]."]
    Enum1 = 0x01,
    #[doc = "clk_1m clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
}
impl OstimerclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OstimerclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OstimerclkselSel {
    #[inline(always)]
    fn from(val: u8) -> OstimerclkselSel {
        OstimerclkselSel::from_bits(val)
    }
}
impl From<OstimerclkselSel> for u8 {
    #[inline(always)]
    fn from(val: OstimerclkselSel) -> u8 {
        OstimerclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PhySel {
    #[doc = "Selects MII PHY Interface."]
    Mii = 0x0,
    #[doc = "Selects RMII PHY Interface."]
    Rmii = 0x01,
}
impl PhySel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PhySel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PhySel {
    #[inline(always)]
    fn from(val: u8) -> PhySel {
        PhySel::from_bits(val)
    }
}
impl From<PhySel> for u8 {
    #[inline(always)]
    fn from(val: PhySel) -> u8 {
        PhySel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PintRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PintRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PintRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PintRst {
    #[inline(always)]
    fn from(val: u8) -> PintRst {
        PintRst::from_bits(val)
    }
}
impl From<PintRst> for u8 {
    #[inline(always)]
    fn from(val: PintRst) -> u8 {
        PintRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PkcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PkcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PkcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PkcRst {
    #[inline(always)]
    fn from(val: u8) -> PkcRst {
        PkcRst::from_bits(val)
    }
}
impl From<PkcRst> for u8 {
    #[inline(always)]
    fn from(val: PkcRst) -> u8 {
        PkcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Pll1clk0divHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divHalt {
        Pll1clk0divHalt::from_bits(val)
    }
}
impl From<Pll1clk0divHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divHalt) -> u8 {
        Pll1clk0divHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Pll1clk0divReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divReset {
        Pll1clk0divReset::from_bits(val)
    }
}
impl From<Pll1clk0divReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divReset) -> u8 {
        Pll1clk0divReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk0divUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Pll1clk0divUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk0divUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk0divUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk0divUnstab {
        Pll1clk0divUnstab::from_bits(val)
    }
}
impl From<Pll1clk0divUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk0divUnstab) -> u8 {
        Pll1clk0divUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Pll1clk1divHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divHalt {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divHalt {
        Pll1clk1divHalt::from_bits(val)
    }
}
impl From<Pll1clk1divHalt> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divHalt) -> u8 {
        Pll1clk1divHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Pll1clk1divReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divReset {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divReset {
        Pll1clk1divReset::from_bits(val)
    }
}
impl From<Pll1clk1divReset> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divReset) -> u8 {
        Pll1clk1divReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pll1clk1divUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Pll1clk1divUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pll1clk1divUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pll1clk1divUnstab {
    #[inline(always)]
    fn from(val: u8) -> Pll1clk1divUnstab {
        Pll1clk1divUnstab::from_bits(val)
    }
}
impl From<Pll1clk1divUnstab> for u8 {
    #[inline(always)]
    fn from(val: Pll1clk1divUnstab) -> u8 {
        Pll1clk1divUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl PllclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivHalt {
        PllclkdivHalt::from_bits(val)
    }
}
impl From<PllclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivHalt) -> u8 {
        PllclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl PllclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivReset {
        PllclkdivReset::from_bits(val)
    }
}
impl From<PllclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivReset) -> u8 {
        PllclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl PllclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivUnstab {
        PllclkdivUnstab::from_bits(val)
    }
}
impl From<PllclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivUnstab) -> u8 {
        PllclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PllclkdivselSel {
    #[doc = "PLL0 clock."]
    Enum0 = 0x0,
    #[doc = "pll1_clk0."]
    Enum1 = 0x01,
    #[doc = "No clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
    #[doc = "No clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "No clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl PllclkdivselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PllclkdivselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PllclkdivselSel {
    #[inline(always)]
    fn from(val: u8) -> PllclkdivselSel {
        PllclkdivselSel::from_bits(val)
    }
}
impl From<PllclkdivselSel> for u8 {
    #[inline(always)]
    fn from(val: PllclkdivselSel) -> u8 {
        PllclkdivselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PluRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PluRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PluRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PluRst {
    #[inline(always)]
    fn from(val: u8) -> PluRst {
        PluRst::from_bits(val)
    }
}
impl From<PluRst> for u8 {
    #[inline(always)]
    fn from(val: PluRst) -> u8 {
        PluRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PortRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PortRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PortRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PortRst {
    #[inline(always)]
    fn from(val: u8) -> PortRst {
        PortRst::from_bits(val)
    }
}
impl From<PortRst> for u8 {
    #[inline(always)]
    fn from(val: PortRst) -> u8 {
        PortRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PqRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PqRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PqRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PqRst {
    #[inline(always)]
    fn from(val: u8) -> PqRst {
        PqRst::from_bits(val)
    }
}
impl From<PqRst> for u8 {
    #[inline(always)]
    fn from(val: PqRst) -> u8 {
        PqRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxI {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCoolfluxI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxI {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxI {
        PriCoolfluxI::from_bits(val)
    }
}
impl From<PriCoolfluxI> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxI) -> u8 {
        PriCoolfluxI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxX {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCoolfluxX {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxX {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxX {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxX {
        PriCoolfluxX::from_bits(val)
    }
}
impl From<PriCoolfluxX> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxX) -> u8 {
        PriCoolfluxX::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCoolfluxYEspi {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCoolfluxYEspi {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCoolfluxYEspi {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCoolfluxYEspi {
    #[inline(always)]
    fn from(val: u8) -> PriCoolfluxYEspi {
        PriCoolfluxYEspi::from_bits(val)
    }
}
impl From<PriCoolfluxYEspi> for u8 {
    #[inline(always)]
    fn from(val: PriCoolfluxYEspi) -> u8 {
        PriCoolfluxYEspi::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Cbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCpu0Cbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Cbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Cbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Cbus {
        PriCpu0Cbus::from_bits(val)
    }
}
impl From<PriCpu0Cbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Cbus) -> u8 {
        PriCpu0Cbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu0Sbus {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCpu0Sbus {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu0Sbus {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu0Sbus {
    #[inline(always)]
    fn from(val: u8) -> PriCpu0Sbus {
        PriCpu0Sbus::from_bits(val)
    }
}
impl From<PriCpu0Sbus> for u8 {
    #[inline(always)]
    fn from(val: PriCpu0Sbus) -> u8 {
        PriCpu0Sbus::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu1CbusSmartDmaI {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCpu1CbusSmartDmaI {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu1CbusSmartDmaI {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu1CbusSmartDmaI {
    #[inline(always)]
    fn from(val: u8) -> PriCpu1CbusSmartDmaI {
        PriCpu1CbusSmartDmaI::from_bits(val)
    }
}
impl From<PriCpu1CbusSmartDmaI> for u8 {
    #[inline(always)]
    fn from(val: PriCpu1CbusSmartDmaI) -> u8 {
        PriCpu1CbusSmartDmaI::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriCpu1SbusSmartDmaD {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriCpu1SbusSmartDmaD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriCpu1SbusSmartDmaD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriCpu1SbusSmartDmaD {
    #[inline(always)]
    fn from(val: u8) -> PriCpu1SbusSmartDmaD {
        PriCpu1SbusSmartDmaD::from_bits(val)
    }
}
impl From<PriCpu1SbusSmartDmaD> for u8 {
    #[inline(always)]
    fn from(val: PriCpu1SbusSmartDmaD) -> u8 {
        PriCpu1SbusSmartDmaD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriNpuD {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriNpuD {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriNpuD {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriNpuD {
    #[inline(always)]
    fn from(val: u8) -> PriNpuD {
        PriNpuD::from_bits(val)
    }
}
impl From<PriNpuD> for u8 {
    #[inline(always)]
    fn from(val: PriNpuD) -> u8 {
        PriNpuD::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriNpuPq {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriNpuPq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriNpuPq {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriNpuPq {
    #[inline(always)]
    fn from(val: u8) -> PriNpuPq {
        PriNpuPq::from_bits(val)
    }
}
impl From<PriNpuPq> for u8 {
    #[inline(always)]
    fn from(val: PriNpuPq) -> u8 {
        PriNpuPq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriPkcEls {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriPkcEls {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriPkcEls {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriPkcEls {
    #[inline(always)]
    fn from(val: u8) -> PriPkcEls {
        PriPkcEls::from_bits(val)
    }
}
impl From<PriPkcEls> for u8 {
    #[inline(always)]
    fn from(val: PriPkcEls) -> u8 {
        PriPkcEls::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbFsEnet {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriUsbFsEnet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbFsEnet {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbFsEnet {
    #[inline(always)]
    fn from(val: u8) -> PriUsbFsEnet {
        PriUsbFsEnet::from_bits(val)
    }
}
impl From<PriUsbFsEnet> for u8 {
    #[inline(always)]
    fn from(val: PriUsbFsEnet) -> u8 {
        PriUsbFsEnet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsbHs {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriUsbHs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsbHs {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsbHs {
    #[inline(always)]
    fn from(val: u8) -> PriUsbHs {
        PriUsbHs::from_bits(val)
    }
}
impl From<PriUsbHs> for u8 {
    #[inline(always)]
    fn from(val: PriUsbHs) -> u8 {
        PriUsbHs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PriUsdhc {
    #[doc = "level 0."]
    Level0 = 0x0,
    #[doc = "level 1."]
    Level1 = 0x01,
    #[doc = "level 2."]
    Level2 = 0x02,
    #[doc = "level 3."]
    Level3 = 0x03,
}
impl PriUsdhc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PriUsdhc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PriUsdhc {
    #[inline(always)]
    fn from(val: u8) -> PriUsdhc {
        PriUsdhc::from_bits(val)
    }
}
impl From<PriUsdhc> for u8 {
    #[inline(always)]
    fn from(val: PriUsdhc) -> u8 {
        PriUsdhc::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Prot(u16);
impl Prot {
    #[doc = "For write operation to have an effect."]
    pub const Prot: Self = Self(0xc0c4);
}
impl Prot {
    pub const fn from_bits(val: u16) -> Prot {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Prot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0xc0c4 => f.write_str("Prot"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prot {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0xc0c4 => defmt::write!(f, "Prot"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Prot {
    #[inline(always)]
    fn from(val: u16) -> Prot {
        Prot::from_bits(val)
    }
}
impl From<Prot> for u16 {
    #[inline(always)]
    fn from(val: Prot) -> u16 {
        Prot::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PufRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl PufRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PufRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PufRst {
    #[inline(always)]
    fn from(val: u8) -> PufRst {
        PufRst::from_bits(val)
    }
}
impl From<PufRst> for u8 {
    #[inline(always)]
    fn from(val: PufRst) -> u8 {
        PufRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Pwm0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm0Rst {
    #[inline(always)]
    fn from(val: u8) -> Pwm0Rst {
        Pwm0Rst::from_bits(val)
    }
}
impl From<Pwm0Rst> for u8 {
    #[inline(always)]
    fn from(val: Pwm0Rst) -> u8 {
        Pwm0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwm1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Pwm1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwm1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwm1Rst {
    #[inline(always)]
    fn from(val: u8) -> Pwm1Rst {
        Pwm1Rst::from_bits(val)
    }
}
impl From<Pwm1Rst> for u8 {
    #[inline(always)]
    fn from(val: Pwm1Rst) -> u8 {
        Pwm1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Qdc0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc0Rst {
    #[inline(always)]
    fn from(val: u8) -> Qdc0Rst {
        Qdc0Rst::from_bits(val)
    }
}
impl From<Qdc0Rst> for u8 {
    #[inline(always)]
    fn from(val: Qdc0Rst) -> u8 {
        Qdc0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Qdc1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Qdc1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Qdc1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Qdc1Rst {
    #[inline(always)]
    fn from(val: u8) -> Qdc1Rst {
        Qdc1Rst::from_bits(val)
    }
}
impl From<Qdc1Rst> for u8 {
    #[inline(always)]
    fn from(val: Qdc1Rst) -> u8 {
        Qdc1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rspt {
    #[doc = "No response when the ETB count expires."]
    NoResp = 0x0,
    #[doc = "Generates a normal interrupt when the ETB count expires."]
    Interrupt = 0x01,
    #[doc = "Generates an NMI interrupt when the ETB count expires."]
    Nmi = 0x02,
    #[doc = "Generates a debug halt when the ETB count expires via CPU0 CTICHIN\\[2\\]."]
    DebugHalt = 0x03,
}
impl Rspt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rspt {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rspt {
    #[inline(always)]
    fn from(val: u8) -> Rspt {
        Rspt::from_bits(val)
    }
}
impl From<Rspt> for u8 {
    #[inline(always)]
    fn from(val: Rspt) -> u8 {
        Rspt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RtcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl RtcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RtcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RtcRst {
    #[inline(always)]
    fn from(val: u8) -> RtcRst {
        RtcRst::from_bits(val)
    }
}
impl From<RtcRst> for u8 {
    #[inline(always)]
    fn from(val: RtcRst) -> u8 {
        RtcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Sai0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0Rst {
    #[inline(always)]
    fn from(val: u8) -> Sai0Rst {
        Sai0Rst::from_bits(val)
    }
}
impl From<Sai0Rst> for u8 {
    #[inline(always)]
    fn from(val: Sai0Rst) -> u8 {
        Sai0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Sai0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivHalt {
        Sai0clkdivHalt::from_bits(val)
    }
}
impl From<Sai0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivHalt) -> u8 {
        Sai0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Sai0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivReset {
        Sai0clkdivReset::from_bits(val)
    }
}
impl From<Sai0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivReset) -> u8 {
        Sai0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Sai0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkdivUnstab {
        Sai0clkdivUnstab::from_bits(val)
    }
}
impl From<Sai0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkdivUnstab) -> u8 {
        Sai0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai0clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "PLL1_CLK0 clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Sai0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai0clkselSel {
        Sai0clkselSel::from_bits(val)
    }
}
impl From<Sai0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai0clkselSel) -> u8 {
        Sai0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Sai1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1Rst {
    #[inline(always)]
    fn from(val: u8) -> Sai1Rst {
        Sai1Rst::from_bits(val)
    }
}
impl From<Sai1Rst> for u8 {
    #[inline(always)]
    fn from(val: Sai1Rst) -> u8 {
        Sai1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Sai1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivHalt {
        Sai1clkdivHalt::from_bits(val)
    }
}
impl From<Sai1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivHalt) -> u8 {
        Sai1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Sai1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivReset {
        Sai1clkdivReset::from_bits(val)
    }
}
impl From<Sai1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivReset) -> u8 {
        Sai1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Sai1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkdivUnstab {
        Sai1clkdivUnstab::from_bits(val)
    }
}
impl From<Sai1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkdivUnstab) -> u8 {
        Sai1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sai1clkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "PLL1_CLK0 clock."]
    Enum4 = 0x04,
    #[doc = "No clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl Sai1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sai1clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sai1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sai1clkselSel {
        Sai1clkselSel::from_bits(val)
    }
}
impl From<Sai1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sai1clkselSel) -> u8 {
        Sai1clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sb3 {
    #[doc = "customer fw load/update file."]
    Customer = 0x0,
    #[doc = "NXP Provisioning FW."]
    Nxp = 0x01,
    #[doc = "ELS signed OEM Provisioning FW."]
    Oem = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sb3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sb3 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sb3 {
    #[inline(always)]
    fn from(val: u8) -> Sb3 {
        Sb3::from_bits(val)
    }
}
impl From<Sb3> for u8 {
    #[inline(always)]
    fn from(val: Sb3) -> u8 {
        Sb3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl SctRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctRst {
    #[inline(always)]
    fn from(val: u8) -> SctRst {
        SctRst::from_bits(val)
    }
}
impl From<SctRst> for u8 {
    #[inline(always)]
    fn from(val: SctRst) -> u8 {
        SctRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl SctclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivHalt {
        SctclkdivHalt::from_bits(val)
    }
}
impl From<SctclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivHalt) -> u8 {
        SctclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl SctclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivReset {
        SctclkdivReset::from_bits(val)
    }
}
impl From<SctclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivReset) -> u8 {
        SctclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl SctclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SctclkdivUnstab {
        SctclkdivUnstab::from_bits(val)
    }
}
impl From<SctclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SctclkdivUnstab) -> u8 {
        SctclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SctclkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum0x3 = 0x03,
    #[doc = "PLL1_clk0 clock."]
    Enum0x4 = 0x04,
    #[doc = "SAI0 MCLK_IN clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
    #[doc = "SAI1 MCLK_IN clock."]
    Enum0x8 = 0x08,
    #[doc = "No clock."]
    Enum0x9 = 0x09,
    #[doc = "No clock."]
    Enum0xA = 0x0a,
    #[doc = "No clock."]
    Enum0xB = 0x0b,
    #[doc = "No clock."]
    Enum0xC = 0x0c,
    #[doc = "No clock."]
    Enum0xD = 0x0d,
    #[doc = "No clock."]
    Enum0xE = 0x0e,
    #[doc = "No clock."]
    Enum0xF = 0x0f,
}
impl SctclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctclkselSel {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctclkselSel {
        SctclkselSel::from_bits(val)
    }
}
impl From<SctclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctclkselSel) -> u8 {
        SctclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sema42Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Sema42Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sema42Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sema42Rst {
    #[inline(always)]
    fn from(val: u8) -> Sema42Rst {
        Sema42Rst::from_bits(val)
    }
}
impl From<Sema42Rst> for u8 {
    #[inline(always)]
    fn from(val: Sema42Rst) -> u8 {
        Sema42Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl SincRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincRst {
    #[inline(always)]
    fn from(val: u8) -> SincRst {
        SincRst::from_bits(val)
    }
}
impl From<SincRst> for u8 {
    #[inline(always)]
    fn from(val: SincRst) -> u8 {
        SincRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincfiltclkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "clk_in."]
    Enum0x2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum0x3 = 0x03,
    #[doc = "FRO_12Mhz clock."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl SincfiltclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SincfiltclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SincfiltclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SincfiltclkselSel {
        SincfiltclkselSel::from_bits(val)
    }
}
impl From<SincfiltclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SincfiltclkselSel) -> u8 {
        SincfiltclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl SlowclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivHalt {
        SlowclkdivHalt::from_bits(val)
    }
}
impl From<SlowclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivHalt) -> u8 {
        SlowclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl SlowclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivReset {
        SlowclkdivReset::from_bits(val)
    }
}
impl From<SlowclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivReset) -> u8 {
        SlowclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SlowclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl SlowclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SlowclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SlowclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> SlowclkdivUnstab {
        SlowclkdivUnstab::from_bits(val)
    }
}
impl From<SlowclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: SlowclkdivUnstab) -> u8 {
        SlowclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sm3Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Sm3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sm3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sm3Rst {
    #[inline(always)]
    fn from(val: u8) -> Sm3Rst {
        Sm3Rst::from_bits(val)
    }
}
impl From<Sm3Rst> for u8 {
    #[inline(always)]
    fn from(val: Sm3Rst) -> u8 {
        Sm3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmartDmaRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl SmartDmaRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmartDmaRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmartDmaRst {
    #[inline(always)]
    fn from(val: u8) -> SmartDmaRst {
        SmartDmaRst::from_bits(val)
    }
}
impl From<SmartDmaRst> for u8 {
    #[inline(always)]
    fn from(val: SmartDmaRst) -> u8 {
        SmartDmaRst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessCpu0SecCode(u32);
impl SwdAccessCpu0SecCode {
    #[doc = "CPU0 DAP is not allowed. Reading back register is read as 0x5."]
    pub const Disable: Self = Self(0x0);
    #[doc = "Value to write to enable CPU0 SWD access. Reading back register is read as 0xA."]
    pub const Enable: Self = Self(0x1234_5678);
}
impl SwdAccessCpu0SecCode {
    pub const fn from_bits(val: u32) -> SwdAccessCpu0SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessCpu0SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disable"),
            0x1234_5678 => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu0SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disable"),
            0x1234_5678 => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessCpu0SecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessCpu0SecCode {
        SwdAccessCpu0SecCode::from_bits(val)
    }
}
impl From<SwdAccessCpu0SecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessCpu0SecCode) -> u32 {
        SwdAccessCpu0SecCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessCpu1SecCode(u32);
impl SwdAccessCpu1SecCode {
    #[doc = "CPU1 DAP is not allowed."]
    pub const Disable: Self = Self(0x0);
    #[doc = "Security code to allow CPU1 DAP."]
    pub const Enable: Self = Self(0x1234_5678);
}
impl SwdAccessCpu1SecCode {
    pub const fn from_bits(val: u32) -> SwdAccessCpu1SecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessCpu1SecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disable"),
            0x1234_5678 => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessCpu1SecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disable"),
            0x1234_5678 => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessCpu1SecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessCpu1SecCode {
        SwdAccessCpu1SecCode::from_bits(val)
    }
}
impl From<SwdAccessCpu1SecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessCpu1SecCode) -> u32 {
        SwdAccessCpu1SecCode::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SwdAccessDspSecCode(u32);
impl SwdAccessDspSecCode {
    #[doc = "DSP DAP is not allowed. Reading back register is read as 0x5."]
    pub const Disable: Self = Self(0x0);
    #[doc = "Value to write to enable DSP SWD access. Reading back register is read as 0xA."]
    pub const Enable: Self = Self(0x1234_5678);
}
impl SwdAccessDspSecCode {
    pub const fn from_bits(val: u32) -> SwdAccessDspSecCode {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for SwdAccessDspSecCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("Disable"),
            0x1234_5678 => f.write_str("Enable"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwdAccessDspSecCode {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "Disable"),
            0x1234_5678 => defmt::write!(f, "Enable"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for SwdAccessDspSecCode {
    #[inline(always)]
    fn from(val: u32) -> SwdAccessDspSecCode {
        SwdAccessDspSecCode::from_bits(val)
    }
}
impl From<SwdAccessDspSecCode> for u32 {
    #[inline(always)]
    fn from(val: SwdAccessDspSecCode) -> u32 {
        SwdAccessDspSecCode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Halt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Systickclkdiv0Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Halt {
        Systickclkdiv0Halt::from_bits(val)
    }
}
impl From<Systickclkdiv0Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Halt) -> u8 {
        Systickclkdiv0Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Reset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Systickclkdiv0Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Reset {
        Systickclkdiv0Reset::from_bits(val)
    }
}
impl From<Systickclkdiv0Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Reset) -> u8 {
        Systickclkdiv0Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv0Unstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Systickclkdiv0Unstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv0Unstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv0Unstab {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv0Unstab {
        Systickclkdiv0Unstab::from_bits(val)
    }
}
impl From<Systickclkdiv0Unstab> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv0Unstab) -> u8 {
        Systickclkdiv0Unstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Halt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Systickclkdiv1Halt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Halt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Halt {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Halt {
        Systickclkdiv1Halt::from_bits(val)
    }
}
impl From<Systickclkdiv1Halt> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Halt) -> u8 {
        Systickclkdiv1Halt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Reset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Systickclkdiv1Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Reset {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Reset {
        Systickclkdiv1Reset::from_bits(val)
    }
}
impl From<Systickclkdiv1Reset> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Reset) -> u8 {
        Systickclkdiv1Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclkdiv1Unstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Systickclkdiv1Unstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclkdiv1Unstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclkdiv1Unstab {
    #[inline(always)]
    fn from(val: u8) -> Systickclkdiv1Unstab {
        Systickclkdiv1Unstab::from_bits(val)
    }
}
impl From<Systickclkdiv1Unstab> for u8 {
    #[inline(always)]
    fn from(val: Systickclkdiv1Unstab) -> u8 {
        Systickclkdiv1Unstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel0Sel {
    #[doc = "SYSTICKCLKDIV0 output."]
    Enum0x0 = 0x0,
    #[doc = "Clk 1 MHz clock."]
    Enum0x1 = 0x01,
    #[doc = "LP Oscillator clock."]
    Enum0x2 = 0x02,
    #[doc = "No clock."]
    Enum0x3 = 0x03,
    #[doc = "No clock."]
    Enum0x4 = 0x04,
    #[doc = "No clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Systickclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel0Sel {
        Systickclksel0Sel::from_bits(val)
    }
}
impl From<Systickclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel0Sel) -> u8 {
        Systickclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Systickclksel1Sel {
    #[doc = "SYSTICKCLKDIV1 output."]
    Enum0x0 = 0x0,
    #[doc = "Clk 1 MHz clock."]
    Enum0x1 = 0x01,
    #[doc = "LP Oscillator clock."]
    Enum0x2 = 0x02,
    #[doc = "No clock."]
    Enum0x3 = 0x03,
    #[doc = "No clock."]
    Enum0x4 = 0x04,
    #[doc = "No clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Systickclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Systickclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Systickclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Systickclksel1Sel {
        Systickclksel1Sel::from_bits(val)
    }
}
impl From<Systickclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Systickclksel1Sel) -> u8 {
        Systickclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer0Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer0Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer0Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer0Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer0Rst {
        Timer0Rst::from_bits(val)
    }
}
impl From<Timer0Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer0Rst) -> u8 {
        Timer0Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer1Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer1Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer1Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer1Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer1Rst {
        Timer1Rst::from_bits(val)
    }
}
impl From<Timer1Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer1Rst) -> u8 {
        Timer1Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer2Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer2Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer2Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer2Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer2Rst {
        Timer2Rst::from_bits(val)
    }
}
impl From<Timer2Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer2Rst) -> u8 {
        Timer2Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer3Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer3Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer3Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer3Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer3Rst {
        Timer3Rst::from_bits(val)
    }
}
impl From<Timer3Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer3Rst) -> u8 {
        Timer3Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Timer4Rst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Timer4Rst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Timer4Rst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Timer4Rst {
    #[inline(always)]
    fn from(val: u8) -> Timer4Rst {
        Timer4Rst::from_bits(val)
    }
}
impl From<Timer4Rst> for u8 {
    #[inline(always)]
    fn from(val: Timer4Rst) -> u8 {
        Timer4Rst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl TraceclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivHalt {
        TraceclkdivHalt::from_bits(val)
    }
}
impl From<TraceclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivHalt) -> u8 {
        TraceclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl TraceclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivReset {
        TraceclkdivReset::from_bits(val)
    }
}
impl From<TraceclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivReset) -> u8 {
        TraceclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl TraceclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> TraceclkdivUnstab {
        TraceclkdivUnstab::from_bits(val)
    }
}
impl From<TraceclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: TraceclkdivUnstab) -> u8 {
        TraceclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TraceclkselSel {
    #[doc = "TRACECLKDIV output."]
    Enum0x0 = 0x0,
    #[doc = "Clk 1 MHz clock."]
    Enum0x1 = 0x01,
    #[doc = "LP Oscillator clock."]
    Enum0x2 = 0x02,
    #[doc = "No clock."]
    Enum0x3 = 0x03,
    #[doc = "No clock."]
    Enum0x4 = 0x04,
    #[doc = "No clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl TraceclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TraceclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TraceclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TraceclkselSel {
        TraceclkselSel::from_bits(val)
    }
}
impl From<TraceclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TraceclkselSel) -> u8 {
        TraceclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl TrngRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrngRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrngRst {
    #[inline(always)]
    fn from(val: u8) -> TrngRst {
        TrngRst::from_bits(val)
    }
}
impl From<TrngRst> for u8 {
    #[inline(always)]
    fn from(val: TrngRst) -> u8 {
        TrngRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TroRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl TroRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TroRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TroRst {
    #[inline(always)]
    fn from(val: u8) -> TroRst {
        TroRst::from_bits(val)
    }
}
impl From<TroRst> for u8 {
    #[inline(always)]
    fn from(val: TroRst) -> u8 {
        TroRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl TsiRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiRst {
    #[inline(always)]
    fn from(val: u8) -> TsiRst {
        TsiRst::from_bits(val)
    }
}
impl From<TsiRst> for u8 {
    #[inline(always)]
    fn from(val: TsiRst) -> u8 {
        TsiRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl TsiclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> TsiclkdivHalt {
        TsiclkdivHalt::from_bits(val)
    }
}
impl From<TsiclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: TsiclkdivHalt) -> u8 {
        TsiclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl TsiclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> TsiclkdivReset {
        TsiclkdivReset::from_bits(val)
    }
}
impl From<TsiclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: TsiclkdivReset) -> u8 {
        TsiclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl TsiclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> TsiclkdivUnstab {
        TsiclkdivUnstab::from_bits(val)
    }
}
impl From<TsiclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: TsiclkdivUnstab) -> u8 {
        TsiclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TsiclkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "No clock."]
    Enum0x1 = 0x01,
    #[doc = "clk_in."]
    Enum0x2 = 0x02,
    #[doc = "No clock."]
    Enum0x3 = 0x03,
    #[doc = "FRO_12Mhz clock."]
    Enum0x4 = 0x04,
    #[doc = "No clock."]
    Enum0x5 = 0x05,
    #[doc = "No clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl TsiclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TsiclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TsiclkselSel {
    #[inline(always)]
    fn from(val: u8) -> TsiclkselSel {
        TsiclkselSel::from_bits(val)
    }
}
impl From<TsiclkselSel> for u8 {
    #[inline(always)]
    fn from(val: TsiclkselSel) -> u8 {
        TsiclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl USdhcclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkdivHalt {
        USdhcclkdivHalt::from_bits(val)
    }
}
impl From<USdhcclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkdivHalt) -> u8 {
        USdhcclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl USdhcclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkdivReset {
        USdhcclkdivReset::from_bits(val)
    }
}
impl From<USdhcclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkdivReset) -> u8 {
        USdhcclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl USdhcclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkdivUnstab {
        USdhcclkdivUnstab::from_bits(val)
    }
}
impl From<USdhcclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkdivUnstab) -> u8 {
        USdhcclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum USdhcclkselSel {
    #[doc = "No clock."]
    Enum0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum1 = 0x01,
    #[doc = "CLKIN clock."]
    Enum2 = 0x02,
    #[doc = "FRO_HF clock."]
    Enum3 = 0x03,
    #[doc = "FRO_12M clock."]
    Enum4 = 0x04,
    #[doc = "pll1_clk1 clock."]
    Enum5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum6 = 0x06,
    #[doc = "No clock."]
    Enum7 = 0x07,
}
impl USdhcclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> USdhcclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for USdhcclkselSel {
    #[inline(always)]
    fn from(val: u8) -> USdhcclkselSel {
        USdhcclkselSel::from_bits(val)
    }
}
impl From<USdhcclkselSel> for u8 {
    #[inline(always)]
    fn from(val: USdhcclkselSel) -> u8 {
        USdhcclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "Updates are allowed to all clock configuration registers."]
    Enable = 0x0,
    #[doc = "Freezes all clock configuration registers update."]
    Freeze = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0FsDcdRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Usb0FsDcdRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0FsDcdRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0FsDcdRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0FsDcdRst {
        Usb0FsDcdRst::from_bits(val)
    }
}
impl From<Usb0FsDcdRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0FsDcdRst) -> u8 {
        Usb0FsDcdRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0FsRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl Usb0FsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0FsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0FsRst {
    #[inline(always)]
    fn from(val: u8) -> Usb0FsRst {
        Usb0FsRst::from_bits(val)
    }
}
impl From<Usb0FsRst> for u8 {
    #[inline(always)]
    fn from(val: Usb0FsRst) -> u8 {
        Usb0FsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Usb0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivHalt {
        Usb0clkdivHalt::from_bits(val)
    }
}
impl From<Usb0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivHalt) -> u8 {
        Usb0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Usb0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivReset {
        Usb0clkdivReset::from_bits(val)
    }
}
impl From<Usb0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivReset) -> u8 {
        Usb0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Usb0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkdivUnstab {
        Usb0clkdivUnstab::from_bits(val)
    }
}
impl From<Usb0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkdivUnstab) -> u8 {
        Usb0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Usb0clkselSel {
    #[doc = "No clock."]
    Enum0x0 = 0x0,
    #[doc = "PLL0 clock."]
    Enum0x1 = 0x01,
    #[doc = "No clock."]
    Enum0x2 = 0x02,
    #[doc = "Clk 48 MHz clock."]
    Enum0x3 = 0x03,
    #[doc = "Clk_in."]
    Enum0x4 = 0x04,
    #[doc = "PLL1_clk0 clock."]
    Enum0x5 = 0x05,
    #[doc = "USB PLL clock."]
    Enum0x6 = 0x06,
    #[doc = "No clock."]
    Enum0x7 = 0x07,
}
impl Usb0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Usb0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Usb0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Usb0clkselSel {
        Usb0clkselSel::from_bits(val)
    }
}
impl From<Usb0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Usb0clkselSel) -> u8 {
        Usb0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsPhyRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UsbHsPhyRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsPhyRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsPhyRst {
    #[inline(always)]
    fn from(val: u8) -> UsbHsPhyRst {
        UsbHsPhyRst::from_bits(val)
    }
}
impl From<UsbHsPhyRst> for u8 {
    #[inline(always)]
    fn from(val: UsbHsPhyRst) -> u8 {
        UsbHsPhyRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsbHsRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UsbHsRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbHsRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbHsRst {
    #[inline(always)]
    fn from(val: u8) -> UsbHsRst {
        UsbHsRst::from_bits(val)
    }
}
impl From<UsbHsRst> for u8 {
    #[inline(always)]
    fn from(val: UsbHsRst) -> u8 {
        UsbHsRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UsdhcRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UsdhcRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsdhcRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsdhcRst {
    #[inline(always)]
    fn from(val: u8) -> UsdhcRst {
        UsdhcRst::from_bits(val)
    }
}
impl From<UsdhcRst> for u8 {
    #[inline(always)]
    fn from(val: UsdhcRst) -> u8 {
        UsdhcRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl UtickRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickRst {
    #[inline(always)]
    fn from(val: u8) -> UtickRst {
        UtickRst::from_bits(val)
    }
}
impl From<UtickRst> for u8 {
    #[inline(always)]
    fn from(val: UtickRst) -> u8 {
        UtickRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl UtickclkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivHalt {
        UtickclkdivHalt::from_bits(val)
    }
}
impl From<UtickclkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivHalt) -> u8 {
        UtickclkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl UtickclkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivReset {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivReset {
        UtickclkdivReset::from_bits(val)
    }
}
impl From<UtickclkdivReset> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivReset) -> u8 {
        UtickclkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl UtickclkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> UtickclkdivUnstab {
        UtickclkdivUnstab::from_bits(val)
    }
}
impl From<UtickclkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: UtickclkdivUnstab) -> u8 {
        UtickclkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum UtickclkselSel {
    #[doc = "clk_in."]
    Enum0 = 0x0,
    #[doc = "xtal32k\\[2\\]."]
    Enum1 = 0x01,
    #[doc = "clk_1m clock."]
    Enum2 = 0x02,
    #[doc = "No clock."]
    Enum3 = 0x03,
}
impl UtickclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickclkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UtickclkselSel {
        UtickclkselSel::from_bits(val)
    }
}
impl From<UtickclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UtickclkselSel) -> u8 {
        UtickclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum VrefRst {
    #[doc = "Block is not reset."]
    Released = 0x0,
    #[doc = "Block is reset."]
    Asserted = 0x01,
}
impl VrefRst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> VrefRst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for VrefRst {
    #[inline(always)]
    fn from(val: u8) -> VrefRst {
        VrefRst::from_bits(val)
    }
}
impl From<VrefRst> for u8 {
    #[inline(always)]
    fn from(val: VrefRst) -> u8 {
        VrefRst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Wdt0clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivHalt {
        Wdt0clkdivHalt::from_bits(val)
    }
}
impl From<Wdt0clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivHalt) -> u8 {
        Wdt0clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Wdt0clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivReset {
        Wdt0clkdivReset::from_bits(val)
    }
}
impl From<Wdt0clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivReset) -> u8 {
        Wdt0clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt0clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Wdt0clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Wdt0clkdivUnstab {
        Wdt0clkdivUnstab::from_bits(val)
    }
}
impl From<Wdt0clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Wdt0clkdivUnstab) -> u8 {
        Wdt0clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivHalt {
    #[doc = "Divider clock is running."]
    Run = 0x0,
    #[doc = "Divider clock is stopped."]
    Halt = 0x01,
}
impl Wdt1clkdivHalt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivHalt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivHalt {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivHalt {
        Wdt1clkdivHalt::from_bits(val)
    }
}
impl From<Wdt1clkdivHalt> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivHalt) -> u8 {
        Wdt1clkdivHalt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivReset {
    #[doc = "Divider is not reset."]
    Released = 0x0,
    #[doc = "Divider is reset."]
    Asserted = 0x01,
}
impl Wdt1clkdivReset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivReset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivReset {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivReset {
        Wdt1clkdivReset::from_bits(val)
    }
}
impl From<Wdt1clkdivReset> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivReset) -> u8 {
        Wdt1clkdivReset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkdivUnstab {
    #[doc = "Divider clock is stable."]
    Stable = 0x0,
    #[doc = "Clock frequency is not stable."]
    Ongoing = 0x01,
}
impl Wdt1clkdivUnstab {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkdivUnstab {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkdivUnstab {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkdivUnstab {
        Wdt1clkdivUnstab::from_bits(val)
    }
}
impl From<Wdt1clkdivUnstab> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkdivUnstab) -> u8 {
        Wdt1clkdivUnstab::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wdt1clkselSel {
    #[doc = "FRO16K clock 2."]
    Enum0 = 0x0,
    #[doc = "fro_hf_div clock."]
    Enum1 = 0x01,
    #[doc = "clk_1m clock."]
    Enum2 = 0x02,
    #[doc = "clk_1m clock."]
    Enum3 = 0x03,
}
impl Wdt1clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt1clkselSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt1clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt1clkselSel {
        Wdt1clkselSel::from_bits(val)
    }
}
impl From<Wdt1clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt1clkselSel) -> u8 {
        Wdt1clkselSel::to_bits(val)
    }
}
