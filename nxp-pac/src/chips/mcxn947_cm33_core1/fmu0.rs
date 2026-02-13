#[doc = "Flash"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmu0 {
    ptr: *mut u8,
}
unsafe impl Send for Fmu0 {}
unsafe impl Sync for Fmu0 {}
impl Fmu0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Flash Status Register"]
    #[inline(always)]
    pub const fn fstat(self) -> crate::common::Reg<regs::Fstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Flash Configuration Register"]
    #[inline(always)]
    pub const fn fcnfg(self) -> crate::common::Reg<regs::Fcnfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Flash Control Register"]
    #[inline(always)]
    pub const fn fctrl(self) -> crate::common::Reg<regs::Fctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob(self, n: usize) -> crate::common::Reg<regs::Fccob, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
