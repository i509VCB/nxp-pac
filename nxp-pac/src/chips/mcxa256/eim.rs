#[doc = "Error Injection Module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eim {
    ptr: *mut u8,
}
unsafe impl Send for Eim {}
unsafe impl Sync for Eim {}
impl Eim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Error Injection Module Configuration Register"]
    #[inline(always)]
    pub const fn eimcr(self) -> crate::common::Reg<regs::Eimcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Error Injection Channel Enable register"]
    #[inline(always)]
    pub const fn eichen(self) -> crate::common::Reg<regs::Eichen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    #[inline(always)]
    pub const fn eichd0_word0(self) -> crate::common::Reg<regs::Eichd0Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    #[inline(always)]
    pub const fn eichd0_word1(self) -> crate::common::Reg<regs::Eichd0Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
pub mod regs;
