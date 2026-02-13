#[doc = "MAUWRAP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mau {
    ptr: *mut u8,
}
unsafe impl Send for Mau {}
unsafe impl Sync for Mau {}
impl Mau {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "System Control"]
    #[inline(always)]
    pub const fn sys_ctlr(self) -> crate::common::Reg<regs::SysCtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "General Exception Status Interrupt Enable"]
    #[inline(always)]
    pub const fn gexp_status_ie(self) -> crate::common::Reg<regs::GexpStatusIe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General Exception Status"]
    #[inline(always)]
    pub const fn gexp_status(self) -> crate::common::Reg<regs::GexpStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Operation Control"]
    #[inline(always)]
    pub const fn op_ctrl(self) -> crate::common::Reg<regs::OpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Result Status Interrupt Enable"]
    #[inline(always)]
    pub const fn res_status_ie(self) -> crate::common::Reg<regs::ResStatusIe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Result Status"]
    #[inline(always)]
    pub const fn res_status(self) -> crate::common::Reg<regs::ResStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Result Register 0"]
    #[inline(always)]
    pub const fn res0(self) -> crate::common::Reg<regs::Res0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Result Register 1"]
    #[inline(always)]
    pub const fn res1(self) -> crate::common::Reg<regs::Res1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Result Register 2"]
    #[inline(always)]
    pub const fn res2(self) -> crate::common::Reg<regs::Res2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Result Register 3"]
    #[inline(always)]
    pub const fn res3(self) -> crate::common::Reg<regs::Res3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
