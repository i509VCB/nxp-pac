#[doc = "FLASH_ROMPATCH."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLASH_ROMPATCH {
    ptr: *mut u8,
}
unsafe impl Send for FLASH_ROMPATCH {}
unsafe impl Sync for FLASH_ROMPATCH {}
impl FLASH_ROMPATCH {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn HEADER(self) -> crate::common::Reg<regs::HEADER, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "."]
    #[inline(always)]
    pub const fn PATCH(self, n: usize) -> crate::common::Reg<regs::PATCH, crate::common::RW> {
        assert!(n < 255usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _) }
    }
}
pub mod regs;
