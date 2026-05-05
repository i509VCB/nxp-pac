#[doc = "ADC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ADC0 {
    ptr: *mut u8,
}
unsafe impl Send for ADC0 {}
unsafe impl Sync for ADC0 {}
impl ADC0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID Register."]
    #[inline(always)]
    pub const fn VERID(self) -> crate::common::Reg<regs::VERID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter Register."]
    #[inline(always)]
    pub const fn PARAM(self) -> crate::common::Reg<regs::PARAM, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "ADC Control Register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "ADC Status Register."]
    #[inline(always)]
    pub const fn STAT(self) -> crate::common::Reg<regs::STAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable Register."]
    #[inline(always)]
    pub const fn IE(self) -> crate::common::Reg<regs::IE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Enable Register."]
    #[inline(always)]
    pub const fn DE(self) -> crate::common::Reg<regs::DE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "ADC Configuration Register."]
    #[inline(always)]
    pub const fn CFG(self) -> crate::common::Reg<regs::CFG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "ADC Pause Register."]
    #[inline(always)]
    pub const fn PAUSE(self) -> crate::common::Reg<regs::PAUSE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Software Trigger Register."]
    #[inline(always)]
    pub const fn SWTRIG(self) -> crate::common::Reg<regs::SWTRIG, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Trigger Status Register."]
    #[inline(always)]
    pub const fn TSTAT(self) -> crate::common::Reg<regs::TSTAT, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "ADC Offset Trim Register."]
    #[inline(always)]
    pub const fn OFSTRIM(self) -> crate::common::Reg<regs::OFSTRIM, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Trigger Control Register."]
    #[inline(always)]
    pub const fn TCTRL(self, n: usize) -> crate::common::Reg<regs::TCTRL, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "FIFO Control Register."]
    #[inline(always)]
    pub const fn FCTRL(self, n: usize) -> crate::common::Reg<regs::FCTRL, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
    #[doc = "Gain Calibration Control."]
    #[inline(always)]
    pub const fn GCC(self, n: usize) -> crate::common::Reg<regs::GCC, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize + n * 4usize) as _) }
    }
    #[doc = "Gain Calculation Result."]
    #[inline(always)]
    pub const fn GCR(self, n: usize) -> crate::common::Reg<regs::GCR, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize + n * 4usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL1(self) -> crate::common::Reg<regs::CMDL1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH1(self) -> crate::common::Reg<regs::CMDH1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL2(self) -> crate::common::Reg<regs::CMDL2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH2(self) -> crate::common::Reg<regs::CMDH2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL3(self) -> crate::common::Reg<regs::CMDL3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH3(self) -> crate::common::Reg<regs::CMDH3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL4(self) -> crate::common::Reg<regs::CMDL4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH4(self) -> crate::common::Reg<regs::CMDH4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL5(self) -> crate::common::Reg<regs::CMDL5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH5(self) -> crate::common::Reg<regs::CMDH5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL6(self) -> crate::common::Reg<regs::CMDL6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH6(self) -> crate::common::Reg<regs::CMDH6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL7(self) -> crate::common::Reg<regs::CMDL7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH7(self) -> crate::common::Reg<regs::CMDH7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL8(self) -> crate::common::Reg<regs::CMDL8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH8(self) -> crate::common::Reg<regs::CMDH8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL9(self) -> crate::common::Reg<regs::CMDL9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH9(self) -> crate::common::Reg<regs::CMDH9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL10(self) -> crate::common::Reg<regs::CMDL10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH10(self) -> crate::common::Reg<regs::CMDH10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL11(self) -> crate::common::Reg<regs::CMDL11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH11(self) -> crate::common::Reg<regs::CMDH11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL12(self) -> crate::common::Reg<regs::CMDL12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH12(self) -> crate::common::Reg<regs::CMDH12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL13(self) -> crate::common::Reg<regs::CMDL13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH13(self) -> crate::common::Reg<regs::CMDH13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL14(self) -> crate::common::Reg<regs::CMDL14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH14(self) -> crate::common::Reg<regs::CMDH14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "ADC Command Low Buffer Register."]
    #[inline(always)]
    pub const fn CMDL15(self) -> crate::common::Reg<regs::CMDL15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "ADC Command High Buffer Register."]
    #[inline(always)]
    pub const fn CMDH15(self) -> crate::common::Reg<regs::CMDH15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "Compare Value Register."]
    #[inline(always)]
    pub const fn CV(self, n: usize) -> crate::common::Reg<regs::CV, crate::common::RW> {
        assert!(n < 4usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Data Result FIFO Register."]
    #[inline(always)]
    pub const fn RESFIFO(self, n: usize) -> crate::common::Reg<regs::RESFIFO, crate::common::R> {
        assert!(n < 2usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize + n * 4usize) as _)
        }
    }
    #[doc = "Calibration General A-Side Registers."]
    #[inline(always)]
    pub const fn CAL_GAR(self, n: usize) -> crate::common::Reg<regs::CAL_GAR, crate::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Calibration General B-Side Registers."]
    #[inline(always)]
    pub const fn CAL_GBR(self, n: usize) -> crate::common::Reg<regs::CAL_GBR, crate::common::RW> {
        assert!(n < 33usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize + n * 4usize) as _)
        }
    }
    #[doc = "ADC Test Register."]
    #[inline(always)]
    pub const fn TST(self) -> crate::common::Reg<regs::TST, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
