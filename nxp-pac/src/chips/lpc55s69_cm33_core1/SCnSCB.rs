#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SCnSCB {
    ptr: *mut u8,
}
unsafe impl Send for SCnSCB {}
unsafe impl Sync for SCnSCB {}
impl SCnSCB {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Coprocessor Power Control Register."]
    #[inline(always)]
    pub const fn CPPWR(self) -> crate::common::Reg<regs::CPPWR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
