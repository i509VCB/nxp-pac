#[doc = "SRC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src {
    ptr: *mut u8,
}
unsafe impl Send for Src {}
unsafe impl Sync for Src {}
impl Src {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRC Control Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SRC Boot Mode Register 1"]
    #[inline(always)]
    pub const fn sbmr1(self) -> crate::common::Reg<regs::Sbmr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SRC Reset Status Register"]
    #[inline(always)]
    pub const fn srsr(self) -> crate::common::Reg<regs::Srsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SRC Boot Mode Register 2"]
    #[inline(always)]
    pub const fn sbmr2(self) -> crate::common::Reg<regs::Sbmr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SRC General Purpose Register 1"]
    #[inline(always)]
    pub const fn gpr1(self) -> crate::common::Reg<regs::Gpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SRC General Purpose Register 2"]
    #[inline(always)]
    pub const fn gpr2(self) -> crate::common::Reg<regs::Gpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SRC General Purpose Register 3"]
    #[inline(always)]
    pub const fn gpr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SRC General Purpose Register 4"]
    #[inline(always)]
    pub const fn gpr4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SRC General Purpose Register 5"]
    #[inline(always)]
    pub const fn gpr5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SRC General Purpose Register 6"]
    #[inline(always)]
    pub const fn gpr6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SRC General Purpose Register 7"]
    #[inline(always)]
    pub const fn gpr7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SRC General Purpose Register 8"]
    #[inline(always)]
    pub const fn gpr8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SRC General Purpose Register 9"]
    #[inline(always)]
    pub const fn gpr9(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SRC General Purpose Register 10"]
    #[inline(always)]
    pub const fn gpr10(self) -> crate::common::Reg<regs::Gpr10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs;
pub mod vals;
