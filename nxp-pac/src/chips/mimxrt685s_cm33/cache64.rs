#[doc = "CACHE64"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache64 {
    ptr: *mut u8,
}
unsafe impl Send for Cache64 {}
unsafe impl Sync for Cache64 {}
impl Cache64 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cache control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Cache line control register"]
    #[inline(always)]
    pub const fn clcr(self) -> crate::common::Reg<regs::Clcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0804usize) as _) }
    }
    #[doc = "Cache search address register"]
    #[inline(always)]
    pub const fn csar(self) -> crate::common::Reg<regs::Csar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0808usize) as _) }
    }
    #[doc = "Cache read/write value register"]
    #[inline(always)]
    pub const fn ccvr(self) -> crate::common::Reg<regs::Ccvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x080cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
