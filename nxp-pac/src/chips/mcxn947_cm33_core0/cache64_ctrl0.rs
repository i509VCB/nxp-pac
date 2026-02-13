#[doc = "CACHE64_CTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache64Ctrl0 {
    ptr: *mut u8,
}
unsafe impl Send for Cache64Ctrl0 {}
unsafe impl Sync for Cache64Ctrl0 {}
impl Cache64Ctrl0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cache Control"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Cache Line Control"]
    #[inline(always)]
    pub const fn clcr(self) -> crate::common::Reg<regs::Clcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Cache Search Address"]
    #[inline(always)]
    pub const fn csar(self) -> crate::common::Reg<regs::Csar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "Cache Read/Write Value"]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
