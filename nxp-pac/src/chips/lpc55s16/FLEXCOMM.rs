#[doc = "Flexcomm serial communication."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FLEXCOMM {
    ptr: *mut u8,
}
unsafe impl Send for FLEXCOMM {}
unsafe impl Sync for FLEXCOMM {}
impl FLEXCOMM {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Peripheral Select and Flexcomm ID register."]
    #[inline(always)]
    pub const fn PSELID(self) -> crate::common::Reg<regs::PSELID, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Peripheral identification register."]
    #[inline(always)]
    pub const fn PID(self) -> crate::common::Reg<regs::PID, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
