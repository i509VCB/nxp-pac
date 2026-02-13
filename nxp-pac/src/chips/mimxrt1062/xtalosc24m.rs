#[doc = "XTALOSC24M"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xtalosc24m {
    ptr: *mut u8,
}
unsafe impl Send for Xtalosc24m {}
unsafe impl Sync for Xtalosc24m {}
impl Xtalosc24m {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0(self) -> crate::common::Reg<regs::Misc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_set(self) -> crate::common::Reg<regs::Misc0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_clr(self) -> crate::common::Reg<regs::Misc0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Miscellaneous Register 0"]
    #[inline(always)]
    pub const fn misc0_tog(self) -> crate::common::Reg<regs::Misc0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl(self) -> crate::common::Reg<regs::LowpwrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl_set(
        self,
    ) -> crate::common::Reg<regs::LowpwrCtrlSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl_clr(
        self,
    ) -> crate::common::Reg<regs::LowpwrCtrlClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "XTAL OSC (LP) Control Register"]
    #[inline(always)]
    pub const fn lowpwr_ctrl_tog(
        self,
    ) -> crate::common::Reg<regs::LowpwrCtrlTog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0(self) -> crate::common::Reg<regs::OscConfig0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0_set(
        self,
    ) -> crate::common::Reg<regs::OscConfig0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0_clr(
        self,
    ) -> crate::common::Reg<regs::OscConfig0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 0 Register"]
    #[inline(always)]
    pub const fn osc_config0_tog(
        self,
    ) -> crate::common::Reg<regs::OscConfig0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1(self) -> crate::common::Reg<regs::OscConfig1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1_set(
        self,
    ) -> crate::common::Reg<regs::OscConfig1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1_clr(
        self,
    ) -> crate::common::Reg<regs::OscConfig1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 1 Register"]
    #[inline(always)]
    pub const fn osc_config1_tog(
        self,
    ) -> crate::common::Reg<regs::OscConfig1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2(self) -> crate::common::Reg<regs::OscConfig2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2_set(
        self,
    ) -> crate::common::Reg<regs::OscConfig2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2_clr(
        self,
    ) -> crate::common::Reg<regs::OscConfig2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "XTAL OSC Configuration 2 Register"]
    #[inline(always)]
    pub const fn osc_config2_tog(
        self,
    ) -> crate::common::Reg<regs::OscConfig2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
}
pub mod regs;
pub mod vals;
