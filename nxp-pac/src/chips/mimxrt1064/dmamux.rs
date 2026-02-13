#[doc = "DMAMUX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmamux {
    ptr: *mut u8,
}
unsafe impl Send for Dmamux {}
unsafe impl Sync for Dmamux {}
impl Dmamux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel index Configuration Register"]
    #[inline(always)]
    pub const fn chcfg(self, n: usize) -> crate::common::Reg<regs::Chcfg, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
