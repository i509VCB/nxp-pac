#[doc = "WWDT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wwdt {
    ptr: *mut u8,
}
unsafe impl Send for Wwdt {}
unsafe impl Sync for Wwdt {}
impl Wwdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Mode"]
    #[inline(always)]
    pub const fn mod_(self) -> crate::common::Reg<regs::Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Timer Constant"]
    #[inline(always)]
    pub const fn tc(self) -> crate::common::Reg<regs::Tc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Feed Sequence"]
    #[inline(always)]
    pub const fn feed(self) -> crate::common::Reg<regs::Feed, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Timer Value"]
    #[inline(always)]
    pub const fn tv(self) -> crate::common::Reg<regs::Tv, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Warning Interrupt Compare Value"]
    #[inline(always)]
    pub const fn warnint(self) -> crate::common::Reg<regs::Warnint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Window Compare Value"]
    #[inline(always)]
    pub const fn window(self) -> crate::common::Reg<regs::Window, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
pub mod vals;
