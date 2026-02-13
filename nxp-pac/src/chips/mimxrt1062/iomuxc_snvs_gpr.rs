#[doc = "IOMUXC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IomuxcSnvsGpr {
    ptr: *mut u8,
}
unsafe impl Send for IomuxcSnvsGpr {}
unsafe impl Sync for IomuxcSnvsGpr {}
impl IomuxcSnvsGpr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPR0 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GPR1 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GPR2 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GPR3 General Purpose Register"]
    #[inline(always)]
    pub const fn gpr3(self) -> crate::common::Reg<regs::Gpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
