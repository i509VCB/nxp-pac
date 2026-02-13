#[doc = "ADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0 {
    ptr: *mut u8,
}
unsafe impl Send for Adc0 {}
unsafe impl Sync for Adc0 {}
impl Adc0 {
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
    #[doc = "ADC Offset Trim Register"]
    #[inline(always)]
    pub const fn ofstrim(self) -> crate::common::Reg<regs::Ofstrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Trigger Control Register"]
    #[inline(always)]
    pub const fn tctrl(self, n: usize) -> crate::common::Reg<regs::Tctrl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fctrl(self, n: usize) -> crate::common::Reg<regs::Fctrl, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
    #[doc = "Gain Calibration Control"]
    #[inline(always)]
    pub const fn gcc(self, n: usize) -> crate::common::Reg<regs::Gcc, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize + n * 4usize) as _) }
    }
    #[doc = "Gain Calculation Result"]
    #[inline(always)]
    pub const fn gcr(self, n: usize) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize + n * 4usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl1(self) -> crate::common::Reg<regs::Cmdl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh1(self) -> crate::common::Reg<regs::Cmdh1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl2(self) -> crate::common::Reg<regs::Cmdl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh2(self) -> crate::common::Reg<regs::Cmdh2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl3(self) -> crate::common::Reg<regs::Cmdl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh3(self) -> crate::common::Reg<regs::Cmdh3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl4(self) -> crate::common::Reg<regs::Cmdl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh4(self) -> crate::common::Reg<regs::Cmdh4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl5(self) -> crate::common::Reg<regs::Cmdl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh5(self) -> crate::common::Reg<regs::Cmdh5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl6(self) -> crate::common::Reg<regs::Cmdl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh6(self) -> crate::common::Reg<regs::Cmdh6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl7(self) -> crate::common::Reg<regs::Cmdl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh7(self) -> crate::common::Reg<regs::Cmdh7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl8(self) -> crate::common::Reg<regs::Cmdl8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh8(self) -> crate::common::Reg<regs::Cmdh8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl9(self) -> crate::common::Reg<regs::Cmdl9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh9(self) -> crate::common::Reg<regs::Cmdh9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl10(self) -> crate::common::Reg<regs::Cmdl10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh10(self) -> crate::common::Reg<regs::Cmdh10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl11(self) -> crate::common::Reg<regs::Cmdl11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh11(self) -> crate::common::Reg<regs::Cmdh11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl12(self) -> crate::common::Reg<regs::Cmdl12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh12(self) -> crate::common::Reg<regs::Cmdh12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl13(self) -> crate::common::Reg<regs::Cmdl13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh13(self) -> crate::common::Reg<regs::Cmdh13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl14(self) -> crate::common::Reg<regs::Cmdl14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh14(self) -> crate::common::Reg<regs::Cmdh14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl15(self) -> crate::common::Reg<regs::Cmdl15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh15(self) -> crate::common::Reg<regs::Cmdh15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
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
    pub const fn resfifo(self, n: usize) -> crate::common::Reg<regs::Resfifo, crate::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "Calibration General A-Side Registers"]
    #[inline(always)]
    pub const fn cal_gar(self, n: usize) -> crate::common::Reg<regs::CalGar, crate::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Calibration General B-Side Registers"]
    #[inline(always)]
    pub const fn cal_gbr(self, n: usize) -> crate::common::Reg<regs::CalGbr, crate::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Test Register"]
    #[inline(always)]
    pub const fn tst(self) -> crate::common::Reg<regs::Tst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
