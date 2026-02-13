#[doc = "CACHE64_POLSEL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cache64Polsel {
    ptr: *mut u8,
}
unsafe impl Send for Cache64Polsel {}
unsafe impl Sync for Cache64Polsel {}
impl Cache64Polsel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Region 0 Top Boundary"]
    #[inline(always)]
    pub const fn reg0_top(self) -> crate::common::Reg<regs::Reg0Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Region 1 Top Boundary"]
    #[inline(always)]
    pub const fn reg1_top(self) -> crate::common::Reg<regs::Reg1Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Policy Select"]
    #[inline(always)]
    pub const fn polsel(self) -> crate::common::Reg<regs::Polsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
