#[doc = "CM7_MCM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7Mcm {
    ptr: *mut u8,
}
unsafe impl Send for Cm7Mcm {}
unsafe impl Sync for Cm7Mcm {}
impl Cm7Mcm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Status and Control Register"]
    #[inline(always)]
    pub const fn iscr(self) -> crate::common::Reg<regs::Iscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs;
