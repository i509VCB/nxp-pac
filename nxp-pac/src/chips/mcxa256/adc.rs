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
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status Register"]
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
    #[doc = "Configuration Register"]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pause Register"]
    #[inline(always)]
    pub const fn pause(self) -> crate::common::Reg<regs::Pause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Software Trigger Register"]
    #[inline(always)]
    pub const fn swtrig(self) -> crate::common::Reg<regs::Swtrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Trigger Status Register"]
    #[inline(always)]
    pub const fn tstat(self) -> crate::common::Reg<regs::Tstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Offset Trim Register"]
    #[inline(always)]
    pub const fn ofstrim(self) -> crate::common::Reg<regs::Ofstrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "High Speed Trim Register"]
    #[inline(always)]
    pub const fn hstrim(self) -> crate::common::Reg<regs::Hstrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Trigger Control Register"]
    #[inline(always)]
    pub const fn tctrl(self, n: usize) -> crate::common::Reg<regs::Tctrl, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fctrl0(self) -> crate::common::Reg<regs::Fctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Gain Calibration Control"]
    #[inline(always)]
    pub const fn gcc0(self) -> crate::common::Reg<regs::Gcc0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Gain Calculation Result"]
    #[inline(always)]
    pub const fn gcr0(self) -> crate::common::Reg<regs::Gcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl1(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh1(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl2(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh2(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl3(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh3(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl4(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh4(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl5(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh5(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl6(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh6(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl7(self) -> crate::common::Reg<regs::Cmdl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh7(self) -> crate::common::Reg<regs::Cmdh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "Compare Value Register"]
    #[inline(always)]
    pub const fn cv(self, n: usize) -> crate::common::Reg<regs::Cv, crate::common::RW> {
        assert!(n < 7usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "Data Result FIFO Register"]
    #[inline(always)]
    pub const fn resfifo0(self) -> crate::common::Reg<regs::Resfifo0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[inline(always)]
    pub const fn cal_gar(self, n: usize) -> crate::common::Reg<regs::CalGar, crate::common::RW> {
        assert!(n < 34usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Configuration 2 Register"]
    #[inline(always)]
    pub const fn cfg2(self) -> crate::common::Reg<regs::Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
