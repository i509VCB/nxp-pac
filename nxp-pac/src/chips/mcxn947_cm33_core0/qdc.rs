#[doc = "ENC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdc {
    ptr: *mut u8,
}
unsafe impl Send for Qdc {}
unsafe impl Sync for Qdc {}
impl Qdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Input Filter"]
    #[inline(always)]
    pub const fn filt(self) -> crate::common::Reg<regs::Filt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Watchdog Timeout"]
    #[inline(always)]
    pub const fn wtr(self) -> crate::common::Reg<regs::Wtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Position Difference Counter"]
    #[inline(always)]
    pub const fn posd(self) -> crate::common::Reg<regs::Posd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Position Difference Hold"]
    #[inline(always)]
    pub const fn posdh(self) -> crate::common::Reg<regs::Posdh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Revolution Counter"]
    #[inline(always)]
    pub const fn rev(self) -> crate::common::Reg<regs::Rev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Revolution Hold"]
    #[inline(always)]
    pub const fn revh(self) -> crate::common::Reg<regs::Revh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Upper Position Counter"]
    #[inline(always)]
    pub const fn upos(self) -> crate::common::Reg<regs::Upos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Lower Position Counter"]
    #[inline(always)]
    pub const fn lpos(self) -> crate::common::Reg<regs::Lpos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Upper Position Hold"]
    #[inline(always)]
    pub const fn uposh(self) -> crate::common::Reg<regs::Uposh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Lower Position Hold"]
    #[inline(always)]
    pub const fn lposh(self) -> crate::common::Reg<regs::Lposh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Upper Initialization"]
    #[inline(always)]
    pub const fn uinit(self) -> crate::common::Reg<regs::Uinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Lower Initialization"]
    #[inline(always)]
    pub const fn linit(self) -> crate::common::Reg<regs::Linit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Input Monitor"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Test"]
    #[inline(always)]
    pub const fn tst(self) -> crate::common::Reg<regs::Tst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Control 2"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Upper Modulus"]
    #[inline(always)]
    pub const fn umod(self) -> crate::common::Reg<regs::Umod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Lower Modulus"]
    #[inline(always)]
    pub const fn lmod(self) -> crate::common::Reg<regs::Lmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Upper Position Compare"]
    #[inline(always)]
    pub const fn ucomp(self) -> crate::common::Reg<regs::Ucomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Lower Position Compare"]
    #[inline(always)]
    pub const fn lcomp(self) -> crate::common::Reg<regs::Lcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Last Edge Time"]
    #[inline(always)]
    pub const fn lastedge(self) -> crate::common::Reg<regs::Lastedge, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Last Edge Time Hold"]
    #[inline(always)]
    pub const fn lastedgeh(self) -> crate::common::Reg<regs::Lastedgeh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Position Difference Period Counter"]
    #[inline(always)]
    pub const fn posdper(self) -> crate::common::Reg<regs::Posdper, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Position Difference Period Buffer"]
    #[inline(always)]
    pub const fn posdperbfr(self) -> crate::common::Reg<regs::Posdperbfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Position Difference Period Hold"]
    #[inline(always)]
    pub const fn posdperh(self) -> crate::common::Reg<regs::Posdperh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Control 3"]
    #[inline(always)]
    pub const fn ctrl3(self) -> crate::common::Reg<regs::Ctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
}
pub mod regs;
pub mod vals;
