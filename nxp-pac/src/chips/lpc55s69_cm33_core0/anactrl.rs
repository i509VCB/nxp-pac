#[doc = "ANALOGCTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anactrl {
    ptr: *mut u8,
}
unsafe impl Send for Anactrl {}
unsafe impl Sync for Anactrl {}
impl Anactrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Various Analog blocks configuration (like FRO 192MHz trimmings source ...)"]
    #[inline(always)]
    pub const fn analog_ctrl_cfg(
        self,
    ) -> crate::common::Reg<regs::AnalogCtrlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Analog Macroblock Identity registers, Flash Status registers"]
    #[inline(always)]
    pub const fn analog_ctrl_status(
        self,
    ) -> crate::common::Reg<regs::AnalogCtrlStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Frequency Measure function control register"]
    #[inline(always)]
    pub const fn freq_me_ctrl(self) -> crate::common::Reg<regs::FreqMeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "192MHz Free Running OScillator (FRO) Control register"]
    #[inline(always)]
    pub const fn fro192m_ctrl(self) -> crate::common::Reg<regs::Fro192mCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "192MHz Free Running OScillator (FRO) Status register"]
    #[inline(always)]
    pub const fn fro192m_status(
        self,
    ) -> crate::common::Reg<regs::Fro192mStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "General Purpose ADC VBAT Divider branch control"]
    #[inline(always)]
    pub const fn adc_ctrl(self) -> crate::common::Reg<regs::AdcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "High speed Crystal Oscillator Control register"]
    #[inline(always)]
    pub const fn xo32m_ctrl(self) -> crate::common::Reg<regs::Xo32mCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "High speed Crystal Oscillator Status register"]
    #[inline(always)]
    pub const fn xo32m_status(self) -> crate::common::Reg<regs::Xo32mStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Brown Out Detectors (BoDs) & DCDC interrupts generation control register"]
    #[inline(always)]
    pub const fn bod_dcdc_int_ctrl(
        self,
    ) -> crate::common::Reg<regs::BodDcdcIntCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "BoDs & DCDC interrupts status register"]
    #[inline(always)]
    pub const fn bod_dcdc_int_status(
        self,
    ) -> crate::common::Reg<regs::BodDcdcIntStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "First Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn ringo0_ctrl(self) -> crate::common::Reg<regs::Ringo0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Second Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn ringo1_ctrl(self) -> crate::common::Reg<regs::Ringo1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Third Ring Oscillator module control register."]
    #[inline(always)]
    pub const fn ringo2_ctrl(self) -> crate::common::Reg<regs::Ringo2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "High Speed Crystal Oscillator (12 MHz - 32 MHz) Voltage Source Supply Control register"]
    #[inline(always)]
    pub const fn ldo_xo32m(self) -> crate::common::Reg<regs::LdoXo32m, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "AUX_BIAS"]
    #[inline(always)]
    pub const fn aux_bias(self) -> crate::common::Reg<regs::AuxBias, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "USB High Speed Phy Control"]
    #[inline(always)]
    pub const fn usbhs_phy_ctrl(self) -> crate::common::Reg<regs::UsbhsPhyCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "USB High Speed Phy Trim values"]
    #[inline(always)]
    pub const fn usbhs_phy_trim(self) -> crate::common::Reg<regs::UsbhsPhyTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
pub mod regs;
pub mod vals;
