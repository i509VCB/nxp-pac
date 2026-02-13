#[doc = "Low-Power Serial Peripheral Interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpspi {
    ptr: *mut u8,
}
unsafe impl Send for Lpspi {}
unsafe impl Sync for Lpspi {}
impl Lpspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn verid(self) -> crate::common::Reg<regs::Verid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Parameter"]
    #[inline(always)]
    pub const fn param(self) -> crate::common::Reg<regs::Param, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn der(self) -> crate::common::Reg<regs::Der, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Configuration 0"]
    #[inline(always)]
    pub const fn cfgr0(self) -> crate::common::Reg<regs::Cfgr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Configuration 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Data Match 0"]
    #[inline(always)]
    pub const fn dmr0(self) -> crate::common::Reg<regs::Dmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Data Match 1"]
    #[inline(always)]
    pub const fn dmr1(self) -> crate::common::Reg<regs::Dmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Clock Configuration"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Clock Configuration 1"]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::common::Reg<regs::Ccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "FIFO Control"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "FIFO Status"]
    #[inline(always)]
    pub const fn fsr(self) -> crate::common::Reg<regs::Fsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Transmit Command"]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Receive Status"]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn rdr(self) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Receive Data Read Only"]
    #[inline(always)]
    pub const fn rdror(self) -> crate::common::Reg<regs::Rdror, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Transmit Command Burst"]
    #[inline(always)]
    pub const fn tcbr(self) -> crate::common::Reg<regs::Tcbr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "Transmit Data Burst"]
    #[inline(always)]
    pub const fn tdbr(self, n: usize) -> crate::common::Reg<regs::Tdbr, crate::common::W> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _)
        }
    }
    #[doc = "Receive Data Burst"]
    #[inline(always)]
    pub const fn rdbr(self, n: usize) -> crate::common::Reg<regs::Rdbr, crate::common::R> {
        assert!(n < 128usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize + n * 4usize) as _)
        }
    }
}
pub mod regs;
pub mod vals;
