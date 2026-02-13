#[doc = "clock ccontroller 1"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkctl1 {
    ptr: *mut u8,
}
unsafe impl Send for Clkctl1 {}
unsafe impl Sync for Clkctl1 {}
impl Clkctl1 {
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
    #[doc = "audio pll0 clock selection"]
    #[inline(always)]
    pub const fn audiopll0clksel(
        self,
    ) -> crate::common::Reg<regs::Audiopll0clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "audio pll0 control0"]
    #[inline(always)]
    pub const fn audiopll0ctl0(self) -> crate::common::Reg<regs::Audiopll0ctl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "audio pll0 lock time"]
    #[inline(always)]
    pub const fn audiopll0locktimediv2(
        self,
    ) -> crate::common::Reg<regs::Audiopll0locktimediv2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "audio pll0 number"]
    #[inline(always)]
    pub const fn audiopll0num(self) -> crate::common::Reg<regs::Audiopll0num, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Audio pll0 denom"]
    #[inline(always)]
    pub const fn audiopll0denom(
        self,
    ) -> crate::common::Reg<regs::Audiopll0denom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "audio pll0 PFD"]
    #[inline(always)]
    pub const fn audiopll0pfd(self) -> crate::common::Reg<regs::Audiopll0pfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "audio pll0 clock divider"]
    #[inline(always)]
    pub const fn audiopllclkdiv(
        self,
    ) -> crate::common::Reg<regs::Audiopllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "DSP cpu clock divider"]
    #[inline(always)]
    pub const fn dspcpuclkdiv(self) -> crate::common::Reg<regs::Dspcpuclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "DSP main ram clock divider"]
    #[inline(always)]
    pub const fn dspmainramclkdiv(
        self,
    ) -> crate::common::Reg<regs::Dspmainramclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "DSP clock selection A"]
    #[inline(always)]
    pub const fn dspcpuclksela(self) -> crate::common::Reg<regs::Dspcpuclksela, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "DSP clock selection B"]
    #[inline(always)]
    pub const fn dspcpuclkselb(self) -> crate::common::Reg<regs::Dspcpuclkselb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "OS EVENT clock selection"]
    #[inline(always)]
    pub const fn oseventfclksel(
        self,
    ) -> crate::common::Reg<regs::Oseventfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "flexcomm clock controller"]
    #[inline(always)]
    pub const fn flexcomm(self, n: usize) -> Flexcomm {
        assert!(n < 8usize);
        unsafe { Flexcomm::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 32usize) as _) }
    }
    #[doc = "FRG clock selection register 14"]
    #[inline(always)]
    pub const fn frg14clksel(self) -> crate::common::Reg<regs::Frg14clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "FRG clock controller 14"]
    #[inline(always)]
    pub const fn frg14ctl(self) -> crate::common::Reg<regs::Frg14ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c4usize) as _) }
    }
    #[doc = "flexcomm14 clock selection"]
    #[inline(always)]
    pub const fn fc14fclksel(self) -> crate::common::Reg<regs::Fc14fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c8usize) as _) }
    }
    #[doc = "FRG clock selection register 15"]
    #[inline(always)]
    pub const fn frg15clksel(self) -> crate::common::Reg<regs::Frg15clksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "FRG clock controller 15"]
    #[inline(always)]
    pub const fn frg15ctl(self) -> crate::common::Reg<regs::Frg15ctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e4usize) as _) }
    }
    #[doc = "flexcomm15 clock selection"]
    #[inline(always)]
    pub const fn fc15fclksel(self) -> crate::common::Reg<regs::Fc15fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e8usize) as _) }
    }
    #[doc = "FRG pll clock divider"]
    #[inline(always)]
    pub const fn frgpllclkdiv(self) -> crate::common::Reg<regs::Frgpllclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06fcusize) as _) }
    }
    #[doc = "DMIC0 clk selection"]
    #[inline(always)]
    pub const fn dmic0fclksel(self) -> crate::common::Reg<regs::Dmic0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "DMIC clock clock divider"]
    #[inline(always)]
    pub const fn dmic0fclkdiv(self) -> crate::common::Reg<regs::Dmic0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "ct32bit timer N clock selection"]
    #[inline(always)]
    pub const fn ct32bitfclksel(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::Ct32bitfclksel, crate::common::RW> {
        assert!(n < 5usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize + n * 4usize) as _)
        }
    }
    #[doc = "audio mclock selection"]
    #[inline(always)]
    pub const fn audiomclksel(self) -> crate::common::Reg<regs::Audiomclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "audio mclock divider"]
    #[inline(always)]
    pub const fn audiomclkdiv(self) -> crate::common::Reg<regs::Audiomclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0744usize) as _) }
    }
    #[doc = "clock out selection 0"]
    #[inline(always)]
    pub const fn clkoutsel0(self) -> crate::common::Reg<regs::Clkoutsel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "clock out selection 1"]
    #[inline(always)]
    pub const fn clkoutsel1(self) -> crate::common::Reg<regs::Clkoutsel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
    #[doc = "clock_out divider"]
    #[inline(always)]
    pub const fn clkoutdiv(self) -> crate::common::Reg<regs::Clkoutdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "I3C0 fclk selection"]
    #[inline(always)]
    pub const fn i3c0fclksel(self) -> crate::common::Reg<regs::I3c0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "I3C0 fclk STC selection"]
    #[inline(always)]
    pub const fn i3c0fclkstcsel(
        self,
    ) -> crate::common::Reg<regs::I3c0fclkstcsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "I3C0 fclk STC divider"]
    #[inline(always)]
    pub const fn i3c0fclkstcdiv(
        self,
    ) -> crate::common::Reg<regs::I3c0fclkstcdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "I3C0 fclks divider"]
    #[inline(always)]
    pub const fn i3c0fclksdiv(self) -> crate::common::Reg<regs::I3c0fclksdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "I3C0 fclk divider"]
    #[inline(always)]
    pub const fn i3c0fclkdiv(self) -> crate::common::Reg<regs::I3c0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "WDT1 clock selection"]
    #[inline(always)]
    pub const fn wdt1fclksel(self) -> crate::common::Reg<regs::Wdt1fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "acomparator 0 clock selection"]
    #[inline(always)]
    pub const fn acmp0fclksel(self) -> crate::common::Reg<regs::Acmp0fclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    #[doc = "acomparator 0 fclk divider"]
    #[inline(always)]
    pub const fn acmp0fclkdiv(self) -> crate::common::Reg<regs::Acmp0fclkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
}
#[doc = "flexcomm clock controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm {
    ptr: *mut u8,
}
unsafe impl Send for Flexcomm {}
unsafe impl Sync for Flexcomm {}
impl Flexcomm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FRG clock selection register N"]
    #[inline(always)]
    pub const fn frgclksel(self) -> crate::common::Reg<regs::Frgclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FRG clock controller"]
    #[inline(always)]
    pub const fn frgctl(self) -> crate::common::Reg<regs::Frgctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "flexcomm clock selection"]
    #[inline(always)]
    pub const fn fcfclksel(self) -> crate::common::Reg<regs::Fcfclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs;
pub mod vals;
