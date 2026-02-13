#[doc = "PUF Key Context Management"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PufCtrl {
    ptr: *mut u8,
}
unsafe impl Send for PufCtrl {}
unsafe impl Sync for PufCtrl {}
impl PufCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PUF command blocking configuration"]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Security level lock"]
    #[inline(always)]
    pub const fn sec_lock(self) -> crate::common::Reg<regs::SecLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Application defined context mask"]
    #[inline(always)]
    pub const fn app_ctx_mask(self) -> crate::common::Reg<regs::AppCtxMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
}
pub mod regs;
pub mod vals;
