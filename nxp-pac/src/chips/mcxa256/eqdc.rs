#[doc = "Quadrature_Decoder"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eqdc {
    ptr: *mut u8,
}
unsafe impl Send for Eqdc {}
unsafe impl Sync for Eqdc {}
impl Eqdc {
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
    #[doc = "Control 2 Register"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "Input Filter Register"]
    #[inline(always)]
    pub const fn filt(self) -> crate::common::Reg<regs::Filt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Last Edge Time Register"]
    #[inline(always)]
    pub const fn lastedge(self) -> crate::common::Reg<regs::Lastedge, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "Position Difference Period Counter Register"]
    #[inline(always)]
    pub const fn posdper(self) -> crate::common::Reg<regs::Posdper, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Position Difference Period Buffer Register"]
    #[inline(always)]
    pub const fn posdperbfr(self) -> crate::common::Reg<regs::Posdperbfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "Upper Position Counter Register"]
    #[inline(always)]
    pub const fn upos(self) -> crate::common::Reg<regs::Upos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Lower Position Counter Register"]
    #[inline(always)]
    pub const fn lpos(self) -> crate::common::Reg<regs::Lpos, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "Position Difference Counter Register"]
    #[inline(always)]
    pub const fn posd(self) -> crate::common::Reg<regs::Posd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Position Difference Hold Register"]
    #[inline(always)]
    pub const fn posdh(self) -> crate::common::Reg<regs::Posdh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Upper Position Hold Register"]
    #[inline(always)]
    pub const fn uposh(self) -> crate::common::Reg<regs::Uposh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Lower Position Hold Register"]
    #[inline(always)]
    pub const fn lposh(self) -> crate::common::Reg<regs::Lposh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "Last Edge Time Hold Register"]
    #[inline(always)]
    pub const fn lastedgeh(self) -> crate::common::Reg<regs::Lastedgeh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Position Difference Period Hold Register"]
    #[inline(always)]
    pub const fn posdperh(self) -> crate::common::Reg<regs::Posdperh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "Revolution Hold Register"]
    #[inline(always)]
    pub const fn revh(self) -> crate::common::Reg<regs::Revh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Revolution Counter Register"]
    #[inline(always)]
    pub const fn rev(self) -> crate::common::Reg<regs::Rev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "Upper Initialization Register"]
    #[inline(always)]
    pub const fn uinit(self) -> crate::common::Reg<regs::Uinit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Lower Initialization Register"]
    #[inline(always)]
    pub const fn linit(self) -> crate::common::Reg<regs::Linit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "Upper Modulus Register"]
    #[inline(always)]
    pub const fn umod(self) -> crate::common::Reg<regs::Umod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Lower Modulus Register"]
    #[inline(always)]
    pub const fn lmod(self) -> crate::common::Reg<regs::Lmod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Upper Position Compare Register 0"]
    #[inline(always)]
    pub const fn ucomp0(self) -> crate::common::Reg<regs::Ucomp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Lower Position Compare Register 0"]
    #[inline(always)]
    pub const fn lcomp0(self) -> crate::common::Reg<regs::Lcomp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Upper Position Compare 1"]
    #[inline(always)]
    pub const fn ucomp1(self) -> crate::common::Reg<regs::Ucomp1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Upper Position Holder Register 1"]
    #[inline(always)]
    pub const fn uposh1(self) -> crate::common::Reg<regs::Uposh1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Lower Position Compare 1"]
    #[inline(always)]
    pub const fn lcomp1(self) -> crate::common::Reg<regs::Lcomp1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Lower Position Holder Register 1"]
    #[inline(always)]
    pub const fn lposh1(self) -> crate::common::Reg<regs::Lposh1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Upper Position Compare 2"]
    #[inline(always)]
    pub const fn ucomp2(self) -> crate::common::Reg<regs::Ucomp2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Upper Position Holder Register 3"]
    #[inline(always)]
    pub const fn uposh2(self) -> crate::common::Reg<regs::Uposh2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Lower Position Compare 2"]
    #[inline(always)]
    pub const fn lcomp2(self) -> crate::common::Reg<regs::Lcomp2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Lower Position Holder Register 2"]
    #[inline(always)]
    pub const fn lposh2(self) -> crate::common::Reg<regs::Lposh2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "Upper Position Compare 3"]
    #[inline(always)]
    pub const fn ucomp3(self) -> crate::common::Reg<regs::Ucomp3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Upper Position Holder Register 3"]
    #[inline(always)]
    pub const fn uposh3(self) -> crate::common::Reg<regs::Uposh3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Lower Position Compare 3"]
    #[inline(always)]
    pub const fn lcomp3(self) -> crate::common::Reg<regs::Lcomp3, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Lower Position Holder Register 3"]
    #[inline(always)]
    pub const fn lposh3(self) -> crate::common::Reg<regs::Lposh3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x36usize) as _) }
    }
    #[doc = "Interrupt Control Register"]
    #[inline(always)]
    pub const fn intctrl(self) -> crate::common::Reg<regs::Intctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Watchdog Timeout Register"]
    #[inline(always)]
    pub const fn wtr(self) -> crate::common::Reg<regs::Wtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3ausize) as _) }
    }
    #[doc = "Input Monitor Register"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Test Register"]
    #[inline(always)]
    pub const fn tst(self) -> crate::common::Reg<regs::Tst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3eusize) as _) }
    }
    #[doc = "Upper VERID"]
    #[inline(always)]
    pub const fn uverid(self) -> crate::common::Reg<regs::Uverid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Lower VERID"]
    #[inline(always)]
    pub const fn lverid(self) -> crate::common::Reg<regs::Lverid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x52usize) as _) }
    }
}
pub mod regs;
pub mod vals;
