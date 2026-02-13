#[doc = "Flexcomm serial communication"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexcomm {
    ptr: *mut u8,
}
unsafe impl Send for Flexcomm {}
unsafe impl Sync for Flexcomm {}
impl Flexcomm {
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
    pub const fn pselid(self) -> crate::common::Reg<regs::Pselid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "Peripheral identification register."]
    #[inline(always)]
    pub const fn pid(self) -> crate::common::Reg<regs::Pid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs;
pub mod vals;
