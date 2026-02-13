#[doc = "IOMUXC_SNVS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IomuxcSnvs {
    ptr: *mut u8,
}
unsafe impl Send for IomuxcSnvs {}
unsafe impl Sync for IomuxcSnvs {}
impl IomuxcSnvs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SW_MUX_CTL_PAD_WAKEUP SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_wakeup(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_PMIC_ON_REQ SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_pmic_on_req(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SW_MUX_CTL_PAD_PMIC_STBY_REQ SW MUX Control Register"]
    #[inline(always)]
    pub const fn sw_mux_ctl_pad_pmic_stby_req(
        self,
    ) -> crate::common::Reg<regs::MuxCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_TEST_MODE SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_test_mode(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadTestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_POR_B SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_por_b(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadPorB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_ONOFF SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_onoff(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadOnoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_WAKEUP SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_wakeup(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadWakeup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_PMIC_ON_REQ SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_pmic_on_req(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadPmicOnReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SW_PAD_CTL_PAD_PMIC_STBY_REQ SW PAD Control Register"]
    #[inline(always)]
    pub const fn sw_pad_ctl_pad_pmic_stby_req(
        self,
    ) -> crate::common::Reg<regs::SwPadCtlPadPmicStbyReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs;
pub mod vals;
