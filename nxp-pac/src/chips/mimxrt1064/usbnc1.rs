#[doc = "USB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbnc1 {
    ptr: *mut u8,
}
unsafe impl Send for Usbnc1 {}
unsafe impl Sync for Usbnc1 {}
impl Usbnc1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB OTG1 Control Register"]
    #[inline(always)]
    pub const fn usb_otg1_ctrl(self) -> crate::common::Reg<regs::UsbOtg1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "OTG1 UTMI PHY Control 0 Register"]
    #[inline(always)]
    pub const fn usb_otg1_phy_ctrl_0(
        self,
    ) -> crate::common::Reg<regs::UsbOtg1PhyCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0818usize) as _) }
    }
}
pub mod regs;
pub mod vals;
