#[doc = "USBNC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs1_usbnc {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs1_usbnc {}
unsafe impl Sync for Usbhs1_usbnc {}
impl Usbhs1_usbnc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB OTG Control 1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB OTG Control 2"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB Host HSIC Control"]
    #[inline(always)]
    pub const fn hsic_ctrl(self) -> crate::common::Reg<regs::HsicCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs;
pub mod vals;
