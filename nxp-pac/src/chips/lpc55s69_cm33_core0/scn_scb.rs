#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScnScb {
    ptr: *mut u8,
}
unsafe impl Send for ScnScb {}
unsafe impl Sync for ScnScb {}
impl ScnScb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Coprocessor Power Control Register"]
    #[inline(always)]
    pub const fn cppwr(self) -> crate::common::Reg<regs::Cppwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
