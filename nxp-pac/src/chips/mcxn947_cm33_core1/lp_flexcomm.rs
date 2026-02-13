#[doc = "LP_FLEXCOMM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpFlexcomm {
    ptr: *mut u8,
}
unsafe impl Send for LpFlexcomm {}
unsafe impl Sync for LpFlexcomm {}
impl LpFlexcomm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Status"]
    #[inline(always)]
    pub const fn istat(self) -> crate::common::Reg<regs::Istat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "Peripheral Select and ID"]
    #[inline(always)]
    pub const fn pselid(self) -> crate::common::Reg<regs::Pselid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
