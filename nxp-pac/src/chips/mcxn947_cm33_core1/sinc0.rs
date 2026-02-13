#[doc = "Prefetch configuration array"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel n Control"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Channel n Data Rate"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Channel n Configuration"]
    #[inline(always)]
    pub const fn ccfr(self) -> crate::common::Reg<regs::Ccfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Channel n Protection"]
    #[inline(always)]
    pub const fn cprot(self) -> crate::common::Reg<regs::Cprot, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Channel n Bias"]
    #[inline(always)]
    pub const fn cbias(self) -> crate::common::Reg<regs::Cbias, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Channel n Low Limit"]
    #[inline(always)]
    pub const fn clolmt(self) -> crate::common::Reg<regs::Clolmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Channel n High Limit"]
    #[inline(always)]
    pub const fn chilmt(self) -> crate::common::Reg<regs::Chilmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Channel n Result Data"]
    #[inline(always)]
    pub const fn crdata(self) -> crate::common::Reg<regs::Crdata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Channel n Multipurpose Data"]
    #[inline(always)]
    pub const fn cmpdata(self) -> crate::common::Reg<regs::Cmpdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Channel n Advanced Configuration"]
    #[inline(always)]
    pub const fn cacfr(self) -> crate::common::Reg<regs::Cacfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Channel n Status"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Channel n Debug"]
    #[inline(always)]
    pub const fn cdbgr(self) -> crate::common::Reg<regs::Cdbgr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
#[doc = "SINC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sinc0 {
    ptr: *mut u8,
}
unsafe impl Send for Sinc0 {}
unsafe impl Sync for Sinc0 {}
impl Sinc0 {
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
    #[doc = "Parameters"]
    #[inline(always)]
    pub const fn parameter(self) -> crate::common::Reg<regs::Parameter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Main Control"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Normal Interrupt Enable"]
    #[inline(always)]
    pub const fn nie(self) -> crate::common::Reg<regs::Nie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Error Interrupt Enable"]
    #[inline(always)]
    pub const fn eie(self) -> crate::common::Reg<regs::Eie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "FIFO And CAD Error Interrupt Enable"]
    #[inline(always)]
    pub const fn fifoie(self) -> crate::common::Reg<regs::Fifoie, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Normal Interrupt Status"]
    #[inline(always)]
    pub const fn nis(self) -> crate::common::Reg<regs::Nis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Error Interrupt Status"]
    #[inline(always)]
    pub const fn eis(self) -> crate::common::Reg<regs::Eis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "FIFO And CAD Error Interrupt Status"]
    #[inline(always)]
    pub const fn fifois(self) -> crate::common::Reg<regs::Fifois, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Status"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Prefetch configuration array"]
    #[inline(always)]
    pub const fn channel(self, n: usize) -> Channel {
        assert!(n < 5usize);
        unsafe { Channel::from_ptr(self.ptr.wrapping_add(0x38usize + n * 48usize) as _) }
    }
}
pub mod regs;
pub mod vals;
