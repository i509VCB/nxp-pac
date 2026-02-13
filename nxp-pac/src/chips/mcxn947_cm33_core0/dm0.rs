#[doc = "DBGMB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dm0 {
    ptr: *mut u8,
}
unsafe impl Send for Dm0 {}
unsafe impl Sync for Dm0 {}
impl Dm0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command and Status Word"]
    #[inline(always)]
    pub const fn csw(self) -> crate::common::Reg<regs::Csw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Request Value"]
    #[inline(always)]
    pub const fn request(self) -> crate::common::Reg<regs::Request, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Return Value"]
    #[inline(always)]
    pub const fn return_(self) -> crate::common::Reg<regs::Return, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Identification"]
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
}
pub mod regs;
