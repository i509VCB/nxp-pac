#[doc = "ADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ADC Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ADC Status Register"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(self) -> crate::common::Reg<regs::Ie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Enable Register"]
    #[inline(always)]
    pub const fn de(self) -> crate::common::Reg<regs::De, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "ADC Configuration Register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "ADC Pause Register"]
    #[inline(always)]
    pub const fn pause(self) -> crate::common::Reg<regs::Pause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "ADC FIFO Control Register"]
    #[inline(always)]
    pub const fn fctrl(self) -> crate::common::Reg<regs::Fctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Software Trigger Register"]
    #[inline(always)]
    pub const fn swtrig(self) -> crate::common::Reg<regs::Swtrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Trigger Control Register"]
    #[inline(always)]
    pub const fn tctrl(self, n: usize) -> crate::common::Reg<regs::Tctrl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl(self, n: usize) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        assert!(n < 15usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 8usize) as _)
        }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh(self, n: usize) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        assert!(n < 15usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize + n * 8usize) as _)
        }
    }
    #[doc = "Compare Value Register"]
    #[inline(always)]
    pub const fn cv(self, n: usize) -> crate::common::Reg<regs::Cv, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Data Result FIFO Register"]
    #[inline(always)]
    pub const fn resfifo(self) -> crate::common::Reg<regs::Resfifo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
}
pub mod regs;
pub mod vals;
