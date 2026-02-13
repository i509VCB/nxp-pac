#[doc = "SAI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai1 {
    ptr: *mut u8,
}
unsafe impl Send for Sai1 {}
unsafe impl Sync for Sai1 {}
impl Sai1 {
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
    #[doc = "Transmit Control"]
    #[inline(always)]
    pub const fn tcsr(self) -> crate::common::Reg<regs::Tcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Transmit Configuration 1"]
    #[inline(always)]
    pub const fn tcr1(self) -> crate::common::Reg<regs::Tcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Transmit Configuration 2"]
    #[inline(always)]
    pub const fn tcr2(self) -> crate::common::Reg<regs::Tcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Transmit Configuration 3"]
    #[inline(always)]
    pub const fn tcr3(self) -> crate::common::Reg<regs::Tcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Transmit Configuration 4"]
    #[inline(always)]
    pub const fn tcr4(self) -> crate::common::Reg<regs::Tcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Transmit Configuration 5"]
    #[inline(always)]
    pub const fn tcr5(self) -> crate::common::Reg<regs::Tcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Transmit Data"]
    #[inline(always)]
    pub const fn tdr(self, n: usize) -> crate::common::Reg<regs::Tdr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Transmit FIFO"]
    #[inline(always)]
    pub const fn tfr(self, n: usize) -> crate::common::Reg<regs::Tfr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Transmit Mask"]
    #[inline(always)]
    pub const fn tmr(self) -> crate::common::Reg<regs::Tmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Receive Control"]
    #[inline(always)]
    pub const fn rcsr(self) -> crate::common::Reg<regs::Rcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Receive Configuration 1"]
    #[inline(always)]
    pub const fn rcr1(self) -> crate::common::Reg<regs::Rcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Receive Configuration 2"]
    #[inline(always)]
    pub const fn rcr2(self) -> crate::common::Reg<regs::Rcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Receive Configuration 3"]
    #[inline(always)]
    pub const fn rcr3(self) -> crate::common::Reg<regs::Rcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Receive Configuration 4"]
    #[inline(always)]
    pub const fn rcr4(self) -> crate::common::Reg<regs::Rcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Receive Configuration 5"]
    #[inline(always)]
    pub const fn rcr5(self) -> crate::common::Reg<regs::Rcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Receive Data"]
    #[inline(always)]
    pub const fn rdr(self, n: usize) -> crate::common::Reg<regs::Rdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "Receive FIFO"]
    #[inline(always)]
    pub const fn rfr(self, n: usize) -> crate::common::Reg<regs::Rfr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Receive Mask"]
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::Rmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
}
pub mod regs;
pub mod vals;
