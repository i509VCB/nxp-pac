#[doc = "LPC-Next0 MUA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mua {
    ptr: *mut u8,
}
unsafe impl Send for Mua {}
unsafe impl Sync for Mua {}
impl Mua {
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
    pub const fn ver(self) -> crate::common::Reg<regs::Ver, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Use Parameter register to determine the parameter settings of MUA."]
    #[inline(always)]
    pub const fn par(self) -> crate::common::Reg<regs::Par, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Transmit Register"]
    #[inline(always)]
    pub const fn tr(self, n: usize) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Receive Register."]
    #[inline(always)]
    pub const fn rr(self, n: usize) -> crate::common::Reg<regs::Rr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
}
pub mod regs;
pub mod vals;
