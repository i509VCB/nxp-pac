#[doc = "CSU registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csu {
    ptr: *mut u8,
}
unsafe impl Send for Csu {}
unsafe impl Sync for Csu {}
impl Csu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Config security level register"]
    #[inline(always)]
    pub const fn csl(self, n: usize) -> crate::common::Reg<regs::Csl, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "HP0 register"]
    #[inline(always)]
    pub const fn hp0(self) -> crate::common::Reg<regs::Hp0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Secure access register"]
    #[inline(always)]
    pub const fn sa(self) -> crate::common::Reg<regs::Sa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "HPCONTROL0 register"]
    #[inline(always)]
    pub const fn hpcontrol0(self) -> crate::common::Reg<regs::Hpcontrol0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
}
pub mod regs;
pub mod vals;
