#[doc = "AIPSTZ Control Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aipstz {
    ptr: *mut u8,
}
unsafe impl Send for Aipstz {}
unsafe impl Sync for Aipstz {}
impl Aipstz {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Master Priviledge Registers"]
    #[inline(always)]
    pub const fn mpr(self) -> crate::common::Reg<regs::Mpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr(self) -> crate::common::Reg<regs::Opacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr1(self) -> crate::common::Reg<regs::Opacr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr2(self) -> crate::common::Reg<regs::Opacr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr3(self) -> crate::common::Reg<regs::Opacr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Off-Platform Peripheral Access Control Registers"]
    #[inline(always)]
    pub const fn opacr4(self) -> crate::common::Reg<regs::Opacr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
}
pub mod regs;
pub mod vals;
