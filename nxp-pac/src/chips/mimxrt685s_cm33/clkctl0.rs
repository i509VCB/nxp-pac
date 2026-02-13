#[doc = "clock ccontroller 0"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkctl0 {
    ptr: *mut u8,
}
unsafe impl Send for Clkctl0 {}
unsafe impl Sync for Clkctl0 {}
impl Clkctl0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "clock control register 0"]
    #[inline(always)]
    pub const fn pscctl0(self) -> crate::common::Reg<regs::Pscctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "clock control register 1"]
    #[inline(always)]
    pub const fn pscctl1(self) -> crate::common::Reg<regs::Pscctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "clock control register 2"]
    #[inline(always)]
    pub const fn pscctl2(self) -> crate::common::Reg<regs::Pscctl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "clock set register 0"]
    #[inline(always)]
    pub const fn pscctl0_set(self) -> crate::common::Reg<regs::Pscctl0Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "clock set register 1"]
    #[inline(always)]
    pub const fn pscctl1_set(self) -> crate::common::Reg<regs::Pscctl1Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "clock set register 2"]
    #[inline(always)]
    pub const fn pscctl2_set(self) -> crate::common::Reg<regs::Pscctl2Set, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "clock clear register 0"]
    #[inline(always)]
    pub const fn pscctl0_clr(self) -> crate::common::Reg<regs::Pscctl0Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "clock clear register 1"]
    #[inline(always)]
    pub const fn pscctl1_clr(self) -> crate::common::Reg<regs::Pscctl1Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "clock clear register 2"]
    #[inline(always)]
    pub const fn pscctl2_clr(self) -> crate::common::Reg<regs::Pscctl2Clr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "FFRO control 0"]
    #[inline(always)]
    pub const fn ffroctl0(self) -> crate::common::Reg<regs::Ffroctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "FFRO control 1"]
    #[inline(always)]
    pub const fn ffroctl1(self) -> crate::common::Reg<regs::Ffroctl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "system oscillator control 0"]
    #[inline(always)]
    pub const fn sysoscctl0(self) -> crate::common::Reg<regs::Sysoscctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "system oscillator bypass"]
    #[inline(always)]
    pub const fn sysoscbypass(self) -> crate::common::Reg<regs::Sysoscbypass, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "low power oscillator control 0"]
    #[inline(always)]
    pub const fn lposcctl0(self) -> crate::common::Reg<regs::Lposcctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "32k oscillator control0"]
    #[inline(always)]
    pub const fn osc32khzctl0(self) -> crate::common::Reg<regs::Osc32khzctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "system pll0 clock selection"]
    #[inline(always)]
    pub const fn syspll0clksel(self) -> crate::common::Reg<regs::Syspll0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "system pll0 control0"]
    #[inline(always)]
    pub const fn syspll0ctl0(self) -> crate::common::Reg<regs::Syspll0ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "system pll0 lock time"]
    #[inline(always)]
    pub const fn syspll0locktimediv2(
        self,
    ) -> crate::common::Reg<regs::Syspll0locktimediv2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "system pll0 number"]
    #[inline(always)]
    pub const fn syspll0num(self) -> crate::common::Reg<regs::Syspll0num, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "system pll0 denom"]
    #[inline(always)]
    pub const fn syspll0denom(self) -> crate::common::Reg<regs::Syspll0denom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "sys pll0 PFD"]
    #[inline(always)]
    pub const fn syspll0pfd(self) -> crate::common::Reg<regs::Syspll0pfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "main pll clk divider"]
    #[inline(always)]
    pub const fn mainpllclkdiv(self) -> crate::common::Reg<regs::Mainpllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "dsp pll clk divider"]
    #[inline(always)]
    pub const fn dsppllclkdiv(self) -> crate::common::Reg<regs::Dsppllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "aux0 pll clk divider"]
    #[inline(always)]
    pub const fn aux0pllclkdiv(self) -> crate::common::Reg<regs::Aux0pllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "aux1 pll clk divider"]
    #[inline(always)]
    pub const fn aux1pllclkdiv(self) -> crate::common::Reg<regs::Aux1pllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "system cpu AHB clock divider"]
    #[inline(always)]
    pub const fn syscpuahbclkdiv(
        self,
    ) -> crate::common::Reg<regs::Syscpuahbclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "main clock selection A"]
    #[inline(always)]
    pub const fn mainclksela(self) -> crate::common::Reg<regs::Mainclksela, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "main clock selection B"]
    #[inline(always)]
    pub const fn mainclkselb(self) -> crate::common::Reg<regs::Mainclkselb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "PFC divider register N"]
    #[inline(always)]
    pub const fn pfcdiv(self, n: usize) -> crate::common::Reg<regs::Pfcdiv, crate::common::RW> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[doc = "FlexSPI FCLK selection"]
    #[inline(always)]
    pub const fn flexspifclksel(
        self,
    ) -> crate::common::Reg<regs::Flexspifclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "FlexSPI FCLK divider"]
    #[inline(always)]
    pub const fn flexspifclkdiv(
        self,
    ) -> crate::common::Reg<regs::Flexspifclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "SCT FCLK selection"]
    #[inline(always)]
    pub const fn sctfclksel(self) -> crate::common::Reg<regs::Sctfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "SCT fclk divider"]
    #[inline(always)]
    pub const fn sctfclkdiv(self) -> crate::common::Reg<regs::Sctfclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    #[doc = "USBHS Fclk selection"]
    #[inline(always)]
    pub const fn usbhsfclksel(self) -> crate::common::Reg<regs::Usbhsfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "USBHS Fclk divider"]
    #[inline(always)]
    pub const fn usbhsfclkdiv(self) -> crate::common::Reg<regs::Usbhsfclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0664usize) as _) }
    }
    #[doc = "SDIO0 FCLK selection"]
    #[inline(always)]
    pub const fn sdio0fclksel(self) -> crate::common::Reg<regs::Sdio0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "SDIO0 FCLK divider"]
    #[inline(always)]
    pub const fn sdio0fclkdiv(self) -> crate::common::Reg<regs::Sdio0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0684usize) as _) }
    }
    #[doc = "SDIO1 FCLK selection"]
    #[inline(always)]
    pub const fn sdio1fclksel(self) -> crate::common::Reg<regs::Sdio1fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "SDIO1 FCLK divider"]
    #[inline(always)]
    pub const fn sdio1fclkdiv(self) -> crate::common::Reg<regs::Sdio1fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0694usize) as _) }
    }
    #[doc = "ESPI clock selection"]
    #[inline(always)]
    pub const fn espifclksel0(self) -> crate::common::Reg<regs::Espifclksel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "ADC0 fclk selection 0"]
    #[inline(always)]
    pub const fn adc0fclksel0(self) -> crate::common::Reg<regs::Adc0fclksel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d0usize) as _) }
    }
    #[doc = "ADC0 fclk selection 1"]
    #[inline(always)]
    pub const fn adc0fclksel1(self) -> crate::common::Reg<regs::Adc0fclksel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d4usize) as _) }
    }
    #[doc = "ADC0 fclk divider"]
    #[inline(always)]
    pub const fn adc0fclkdiv(self) -> crate::common::Reg<regs::Adc0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d8usize) as _) }
    }
    #[doc = "UTICK fclk selection"]
    #[inline(always)]
    pub const fn utickfclksel(self) -> crate::common::Reg<regs::Utickfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "wdt clock selection"]
    #[inline(always)]
    pub const fn wdt0fclksel(self) -> crate::common::Reg<regs::Wdt0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "32k wake clock selection"]
    #[inline(always)]
    pub const fn wakeclk32khzsel(
        self,
    ) -> crate::common::Reg<regs::Wakeclk32khzsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "32k wake clock divider"]
    #[inline(always)]
    pub const fn wakeclk32khzdiv(
        self,
    ) -> crate::common::Reg<regs::Wakeclk32khzdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "system tick fclk selection"]
    #[inline(always)]
    pub const fn systickfclksel(
        self,
    ) -> crate::common::Reg<regs::Systickfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "system tick fclk divider"]
    #[inline(always)]
    pub const fn systickfclkdiv(
        self,
    ) -> crate::common::Reg<regs::Systickfclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
}
pub mod regs;
pub mod vals;
