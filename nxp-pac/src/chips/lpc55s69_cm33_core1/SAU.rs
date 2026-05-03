#[doc = "no description available."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SAU {
    ptr: *mut u8,
}
unsafe impl Send for SAU {}
unsafe impl Sync for SAU {}
impl SAU {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Security Attribution Unit Control Register."]
    #[inline(always)]
    pub const fn CTRL(self) -> crate::common::Reg<regs::CTRL, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "Security Attribution Unit Type Register."]
    #[inline(always)]
    pub const fn TYPE(self) -> crate::common::Reg<regs::TYPE, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Security Attribution Unit Region Number Register."]
    #[inline(always)]
    pub const fn RNR(self) -> crate::common::Reg<regs::RNR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Security Attribution Unit Region Base Address Register."]
    #[inline(always)]
    pub const fn RBAR(self) -> crate::common::Reg<regs::RBAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Security Attribution Unit Region Limit Address Register."]
    #[inline(always)]
    pub const fn RLAR(self) -> crate::common::Reg<regs::RLAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Secure Fault Status Register."]
    #[inline(always)]
    pub const fn SFSR(self) -> crate::common::Reg<regs::SFSR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Secure Fault Address Register."]
    #[inline(always)]
    pub const fn SFAR(self) -> crate::common::Reg<regs::SFAR, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
