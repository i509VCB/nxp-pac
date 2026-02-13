#[doc = "LPC-Next0 SEMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sema42 {
    ptr: *mut u8,
}
unsafe impl Send for Sema42 {}
unsafe impl Sync for Sema42 {}
impl Sema42 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Semphores2 Gate n"]
    #[inline(always)]
    pub const fn gate(self, n: usize) -> crate::common::Reg<regs::Gate, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 1usize) as _) }
    }
    #[doc = "Reset Gate Read"]
    #[inline(always)]
    pub const fn rstgt_r(self) -> crate::common::Reg<regs::RstgtR, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
    #[doc = "Reset Gate Write"]
    #[inline(always)]
    pub const fn rstgt_w(self) -> crate::common::Reg<regs::RstgtW, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x42usize) as _) }
    }
}
pub mod regs;
pub mod vals;
