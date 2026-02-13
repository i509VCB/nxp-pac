#[doc = "QDC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enc {
    ptr: *mut u8,
}
unsafe impl Send for Enc {}
unsafe impl Sync for Enc {}
impl Enc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Input Filter Register"]
    #[inline(always)]
    pub const fn filt(self) -> crate::common::Reg<regs::Filt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Watchdog Timeout Register"]
    #[inline(always)]
    pub const fn wtr(self) -> crate::common::Reg<regs::Wtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Position Difference Counter Register"]
    #[inline(always)]
    pub const fn posd(self) -> crate::common::Reg<regs::Posd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Position Difference Hold Register"]
    #[inline(always)]
    pub const fn posdh(self) -> crate::common::Reg<regs::Posdh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Revolution Counter Register"]
    #[inline(always)]
    pub const fn rev(self) -> crate::common::Reg<regs::Rev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Revolution Hold Register"]
    #[inline(always)]
    pub const fn revh(self) -> crate::common::Reg<regs::Revh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Upper Position Counter Register"]
    #[inline(always)]
    pub const fn upos(self) -> crate::common::Reg<regs::Upos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Lower Position Counter Register"]
    #[inline(always)]
    pub const fn lpos(self) -> crate::common::Reg<regs::Lpos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Upper Position Hold Register"]
    #[inline(always)]
    pub const fn uposh(self) -> crate::common::Reg<regs::Uposh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Lower Position Hold Register"]
    #[inline(always)]
    pub const fn lposh(self) -> crate::common::Reg<regs::Lposh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Upper Initialization Register"]
    #[inline(always)]
    pub const fn uinit(self) -> crate::common::Reg<regs::Uinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Lower Initialization Register"]
    #[inline(always)]
    pub const fn linit(self) -> crate::common::Reg<regs::Linit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Input Monitor Register"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Test Register"]
    #[inline(always)]
    pub const fn tst(self) -> crate::common::Reg<regs::Tst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Upper Modulus Register"]
    #[inline(always)]
    pub const fn umod(self) -> crate::common::Reg<regs::Umod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Lower Modulus Register"]
    #[inline(always)]
    pub const fn lmod(self) -> crate::common::Reg<regs::Lmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Upper Position Compare Register"]
    #[inline(always)]
    pub const fn ucomp(self) -> crate::common::Reg<regs::Ucomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Lower Position Compare Register"]
    #[inline(always)]
    pub const fn lcomp(self) -> crate::common::Reg<regs::Lcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
}
pub mod regs;
pub mod vals;
